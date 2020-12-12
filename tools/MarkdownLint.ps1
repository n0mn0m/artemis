$global:LASTEXITCODE = 0
$command = 'markdownlint ./**/*.md --config ./tools/.markdownlint.json'
Invoke-Expression $command
exit $LASTEXITCODE