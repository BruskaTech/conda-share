$ErrorActionPreference = "Stop"

if (-not $env:TARGET) { throw "TARGET is not set" }

$tag = if ($env:GITHUB_REF_NAME) { $env:GITHUB_REF_NAME } else { "local" }
$distDir = "dist"

# Get metadata once
$metaJson = cargo metadata --no-deps --format-version=1 | Out-String
$meta = $metaJson | ConvertFrom-Json

$targetDir = $meta.target_directory
if (-not $targetDir) { throw "Could not determine target_directory from cargo metadata." }

$releaseDir = Join-Path $targetDir (Join-Path $env:TARGET "release")

Write-Host "Target directory: $targetDir"
Write-Host "Release directory: $releaseDir"

# Find all workspace binaries (package, bin)
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

# Build ONCE
Write-Host ""
Write-Host "== Building workspace once: --release --target $env:TARGET =="
cargo build --release --workspace --target $env:TARGET

# Package all bins from release dir
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

foreach ($p in $pairs) {
  $pkg = $p.pkg
  $bin = $p.bin

  $binPath = Join-Path $releaseDir "$bin.exe"
  if (-not (Test-Path $binPath)) {
    throw "Expected binary not found: $binPath"
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
