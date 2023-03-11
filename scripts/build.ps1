param (
    [string]$type = "release"
)
Write-Output "Building native dependencies with profile $type."
Write-Output "Recreating device folder..."
if (Test-Path ".\devices") {
    Write-Output "Device folder exists! Deleting."
    Remove-Item ".\devices" -Force -Recurse
}
New-Item "devices" -ItemType Directory | Out-Null
$files = Get-ChildItem ".\devices-src"
foreach ($f in $files) {
    Set-Location $f.FullName
    $output = Split-Path $f.FullName -leaf
    Write-Output "Compiling device $output..."
    if ($type -eq "release") {
        npm run build-release
    }
    else {
        npm run build-debug
    }
    Write-Output "Copying artifact $output.node..."
    New-Item "../../devices/$output" -ItemType Directory | Out-Null
    Copy-Item -Path ".\$output.node" -Destination "../../devices/$output/deviceplugin.node"
    Write-Output "Copying device plugin resources..."
    Copy-Item -Path ".\resources" -Destination "../../devices/$output" -Recurse
}
Set-Location "../../"