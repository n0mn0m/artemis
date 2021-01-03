param(
    [Parameter(Mandatory=$True, Position=0, ValueFromPipeline=$false)]
    [System.String]
    $Command
)

$global:LASTEXITCODE = 0

$CargoMakeCommand = "cargo make ${Command}"
Write-Host $CargoMakeCommand
Invoke-Expression $CargoMakeCommand
exit $LASTEXITCODE
