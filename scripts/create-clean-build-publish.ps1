Write-Output "Creating clean release folder for packaging..."
if (Test-Path ".\clean-folder") {
    Write-Output "Clean folder exists! Deleting..."
    Remove-Item ".\clean-folder" -Force -Recurse
}
New-Item "clean-folder" -ItemType Directory | Out-Null
Copy-Item -Path ".\assets" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\build" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\devices" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\libraries" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\node_modules" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\themes" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\app.config.json" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\forge.config.js" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\LICENSE" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\main.js" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\package.json" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\package-lock.json" -Destination ".\clean-folder" -Recurse
Copy-Item -Path ".\startup.js" -Destination ".\clean-folder" -Recurse
Set-Location ".\clean-folder"
npm run create-package
Write-Output "Created package."
npx electron-forge publish
Write-Output "Published package."
Set-Location "..\"
if (Test-Path ".\out") {
    Write-Output "Out folder exists! Deleting..."
    Remove-Item ".\out" -Force -Recurse
}
Copy-Item -Path ".\clean-folder\out" -Destination ".\out" -Recurse
Write-Output "Copied package."
Remove-Item ".\clean-folder" -Force -Recurse
Write-Output "Clean folder deleted."