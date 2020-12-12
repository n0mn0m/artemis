function Test-AzDOPipeline {
    <#
    .SYNOPSIS
    Tests an Azure Devops Pipeline YAML configuration
    .DESCRIPTION
    This can be used to validate an Azure Devops pipeline configuration within a particular pipeline project.
    #>
    param (
        #Your Azure Devops Organization Name
        [Parameter(Mandatory)]$Organization,
        #Your Azure Devops Project
        [Parameter(Mandatory)]$Project,
        #Your Azure Devops Project Pipeline ID
        [Parameter(Mandatory)]$PipelineID,
        #Optional Path to the YAML File you wish to validate. This will override the existing pipeline configuration.
        [String]$Path,
        #Your Azure Devops Personal Access Token as a secure string. If not supplied you will be prompted for it. More Info: https://tinyurl.com/AZDOPAT
        [SecureString]$PersonalAccessToken
    )
    $ErrorActionPreference = 'Stop'

    if (-not $PersonalAccessToken) {[SecureString]$PersonalAccessToken = Read-Host -Prompt 'Azure Devops Personal Access Token' -AsSecureString}

    $PATBase64 = [System.Convert]::ToBase64String(
        [System.Text.Encoding]::ASCII.GetBytes(
            ":$([Net.NetworkCredential]::new('PAT', $PersonalAccessToken).password)"
        )
    )

    $InvokeRestParams = @{
        ContentType = 'application/json'
        Uri         = "https://dev.azure.com/$Organization/$Project/_apis/pipelines/$PipelineId/runs?api-version=5.1-preview"
        Method      = 'POST'
        Headers     = @{
            Authorization = "Basic $PATBase64"
        }
        Body        = @{
            PreviewRun = $true
        }
    }
    if ($Path) {
        $InvokeRestParams.Body.YamlOverride = [string](Get-Content -raw $Path)
    }
    $InvokeRestParams.Body = $InvokeRestParams.Body | ConvertTo-Json
    try {
        $result = Invoke-RestMethod @InvokeRestParams -ErrorAction Stop
    } catch {
        if ($PSItem -match 'PipelineValidationException') {
            Write-Error (($PSItem | ConvertFrom-Json).message -replace '/azure-pipelines.yml( ?: ?)? ?','')
            return
        } else {
            throw
        }
    }
    if ($Result -match 'Azure DevOps Services \| Sign In') {
        Write-Error 'Authentication failed. Please check that your Personal Access Token is correct.
More Info: https://docs.microsoft.com/en-us/azure/devops/organizations/accounts/use-personal-access-tokens-to-authenticate'
        return
    }
    write-host -ForegroundColor green 'Pipeline YAML is VALID. Expanded result:'
    write-host -ForegroundColor green '----------------------------------------'

    $result.finalYaml
}
