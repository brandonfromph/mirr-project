Set-Location C:\Users\elvie\nasa-rust-project

cargo.exe run -p lra-cli -- build-docs docs -o _site

New-Item -ItemType Directory -Force _site\assets\images | Out-Null
New-Item -ItemType Directory -Force _site\paper\demos | Out-Null

Copy-Item docs\assets\images\* _site\assets\images\ -Recurse -Force -ErrorAction SilentlyContinue
Copy-Item paper\index.html,paper\paper.css,paper\paper.js,paper\sw.js _site\paper\ -Force
Copy-Item paper\lra-card.svg,paper\lra-client.js _site\paper\ -Force
Copy-Item paper\LICENSE,paper\CITATION.cff _site\paper\ -Force
Copy-Item demos\mirr_wasm.js,demos\mirr_wasm_bg.wasm _site\paper\demos\ -Force

$hash = (git rev-parse --short HEAD).Trim()
$index = Get-Content _site\paper\index.html -Raw
$index = $index -replace 'id="commit-hash">dev', ('id="commit-hash">' + $hash)
$index = $index -replace 'id="footer-hash">dev', ('id="footer-hash">' + $hash)
Set-Content _site\paper\index.html $index

Write-Output "Assembled site with commit=$hash"

$required = @(
  '_site\index.html',
  '_site\style.css',
  '_site\paper\index.html',
  '_site\paper\paper.js',
  '_site\paper\demos\mirr_wasm.js',
  '_site\paper\demos\mirr_wasm_bg.wasm'
)

foreach ($f in $required) {
  if (-not (Test-Path $f)) {
    throw "Missing required artifact: $f"
  }
}

Write-Output 'Site verification OK'
Get-Item _site\paper\demos\mirr_wasm.js,_site\paper\demos\mirr_wasm_bg.wasm | Select-Object Name,Length,LastWriteTime
