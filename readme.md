
vscode windows 运行
首先在 PowerShell 下设置环境变量
$env:RUST_BACKTRACE=1 ;
$env:profile= "dev" ;

cargo run --bin ruoyi_rust_dev

$env:RUST_BACKTRACE=1 ;
$env:profile= "prod" ;

cargo run --bin ruoyi_rust_prod



