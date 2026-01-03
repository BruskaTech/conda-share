$ErrorActionPreference = "Stop"

if (-not $env:TARGET) { throw "TARGET is not set" }

$tag = if ($env:GITHUB_REF_NAME) { $env:GITHUB_REF_NAME } else { "local" }
$distDir = "dist"

$metaJson = cargo metadata --no-deps --format-version=1 | Out-String
$meta = $metaJson | ConvertFrom-Json

$pairs = @()
foreach ($pkg in $meta.packages) {
  foreach ($t in $pkg.targets) {
    if ($t.kind -contains "bin") {
      $pairs += [PSCustomObject]@{ pkg = $pkg.name; bin = $t.name }
    }
  }
}

if ($pairs.Count -eq 0) {
  throw "No workspace binaries found (no targets with kind=bin)."
}

Write-Host "Found $($pairs.Count) binaries in workspace."
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

foreach ($p in $pairs) {
  $pkg = $p.pkg
  $bin = $p.bin

  Write-Host ""
  Write-Host "== Building: package=$pkg bin=$bin target=$env:TARGET =="

  cargo build --release --target $env:TARGET -p $pkg --bin $bin

  $binPath = "target\$env:TARGET\release\$bin.exe"
  if (-not (Test-Path $binPath)) {
    throw "Expected binary not found at $binPath"
  }

  $zipName = "$bin-$tag-$env:TARGET.zip"
  $binOutDir = Join-Path $distDir $bin
  New-Item -ItemType Directory -Force -Path $binOutDir | Out-Null

  Copy-Item $binPath -Destination (Join-Path $binOutDir "$bin.exe") -Force

  if (Test-Path $zipName) { Remove-Item $zipName -Force }
  Compress-Archive -Path (Join-Path $binOutDir "$bin.exe") -DestinationPath $zipName

  Write-Host "Created $zipName"
}

Write-Host ""
Write-Host "Done. Zips in repo root:"
Get-ChildItem *.zip | Select-Object -ExpandProperty Name
