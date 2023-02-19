use ethers::prelude::*;
use std::sync::Arc;
use eyre::Result;
use ethers_providers::{Provider, Http, Middleware};
use ethers::utils::format_units;
use std::convert::TryFrom;
use console::Emoji;
use clipboard::{ClipboardContext, ClipboardProvider};
use dotenv;
#[macro_use]
extern crate dotenv_codegen;


#[tokio::main]
async fn main() -> eyre::Result<()> {
   
    get_builders().await?;
    Ok(())
}

abigen!(
    STREAM_CONTRACT,
    r#"[
        function streamBalance() external view returns(uint256)
        function cap() external view returns(uint256)
    ]"#,
);

async fn get_builders() -> Result<()> {
    dotenv::dotenv().ok();
    // Get the mainnet rpc url from .env file in root directory . See .example.env
    let rpc = dotenv!("RPC");
    let rpc_url: &str = rpc;
    // Stream contract adddresses list . 
    let mut address_array:[&str;68]=  ["0xfF12a20cB04FC55B424a63036E72533b8F3D005C","0xA6E9462adf5419195c85e81fb130c3E053348873",
    "0xE5C281c470AcedD6f15d41C640988822594bf69A","0x0BB42B100Fff13f6C488Fd6CdB608219c0f3Fe1F",
    "0x4Ea27d0c3133217b46a72532C89D77557A531952","0x3CE33e31d298da1c3e66807635F82D86CC174ef1","0xd03091A70708B457265B2cBdCC3Be7748700F7f0",
    "0x1fa548259672f607182FBF000ed562E8Ea6a36A4","0x6D6A108C742509Dd6B1019347f0f4ADCeE040358"
    ,"0x18643bd918b26936161099c0C668a8bb394B38B9","0x7F17Eda90783fE69be962A224bBcddd9Ca4213a3",
    "0xfa43D87DD4Ed679c9aEd282d7901DAF182C748F8","0xd0122C6e073591A7d70991fA809e795EF7c848A7",
    "0x8EE7FDe7B06C6b5511a788CdE1d197b39DF8df7f","0x24aAc13141DbE8946433215bfdc793C2B71398c8",
    "0x52864De6545554437999FA20374AFf409B4F52b7","0x12E1F6Bd5E832dAc56889230553B06b7FEA4A009",
    "0x4cc7976d1b0784808E838CD89e0A4dF957B0f652","0x864Fa2F20e414c9534B1DE567a30a77436c7a745",
    "0xbE6c8116D19433068330c403c804b54B75a101C7","0x139c9689D8d778157eCb63EE907C6e4428015ea3",
    "0x97f29525cF5d1b673eBb296ca717F15cDEd5F034","0x1051D98d1b01b486dACce307acCd12cC0D01a216",
    "0xc3c9ff28Ffa2BB65E5827C5Fdc309FFC41e5017e","0x1eB6Da6F03B6D3C0d8da0B127388Add4d78Eb652",
    "0x619ACcbE6E5C4E5Cc71a29a05eE7228867c9733c","0xc982728F598DC3De85829C46F5Aa0c791BdeEe4f",
    "0x99502d8d0F07cF8D60ef27fa9557121626237dDD","0xc80bfd26B102991E2D96CE583B5eFA2E4Db0733d",
    "0x864Fa2F20e414c9534B1DE567a30a77436c7a745","0xEE241aE052765dE18675A04aeE8F92c0df31e445",
    "0xB0d15EaDC7C28672B280C8323D8C4906c774D589","0x8Ab22Df5FADAeF50486949576baED1F5f058631A",
    "0x8601d2C0c0dc9E2f63f6a75b8Dc986A22F3a039A","0xb55E52478E9d7C515e2459F95CDf7Df692C44549",
    "0x86c6C2c9699bE74278E0d73065fF12249221Bd30","0xDbcD66b510191cD0539F7FAe8cD981B82Ee2006f",
    "0x63f4eE9A7557d1A6432f32E7f101860fc1485f99","0x94EB46649152e9187Ffd18FEb29ADb5FAC3C6405",
    "0x3759fD32297f20F1e1E778479d935cC940C05E5c","0xD3106575c2eB4D259D52a46F231777b0d8Bb1a10",
    "0x9E0404618b10838fF466C98D87839122629669C7","0x8429191DA3946f6B5775149e08267B3458431423",
    "0x61f7e4A14CFc78A6f6B2Ac46219328b76214A712","0xCf1cf8e38767cfce6e7ab46EBCd059efF20B0558",
    "0xbb8C20Bd6224AC33fBd6C73BdCcaB96a080D26E9","0x42BDA3178b162A586E6e864747C90189996ceFFd",
    "0x7d5a72B17C1aC153C50E11cCBe756859782e3A49","0xED9FdAE176d2254a43A61ab5635d21b920384E13",
    "0x707a07A65A466A89A9815027D79B2F30359D6A02","0x518Af5F20bf07C882e17731207761C174AB4F9c4",
    "0x0A9eDE9A66683F23d369FC6bAAA9D1fa7198Ebf2","0xC74a1Cb3715De1D82182816582bb330d5086B081",
    "0x560Dd59ED235446d04da7C907289E3f88e685447","0x9B32EFA35C525Facd5f6a152787f0eeC6F2D8521",
    "0xC89eaBBD19043151d354Bf3ecfEd64540b95dD14","0x341E1A2859B2e980C966cFAF8055Ea28d531cE9D",
    "0x69701Be2Bfd799C60F16Fa0e941F99EEEFA4CD24","0xf93bc4114c68F73566a2f14A80b09F928bAD914C",
    "0x85bDD3FFac4e02a884F05423A3C2b02d004E57aB","0x61d7De768468451888A110Db7b27F12B9423b6a6",
    "0xb32270518664C77a09E44f6DA59aD2dd3470299C","0xe4a422eEF16bd605B8B065b59d175799A9DC2c71",
    "0x5090E445920f304aB66070c94eEd9F0Ff38728B4","0x130A49071284a770fD07d6aDACA8b23d4aFBAdd0",
    "0x95111aaC2028687Ad84592279270111aF75C4824","0xfB85B0F69178c4bFDbeAe802B06ca0c0485870ED",
    "0xC0EB2fb2A3922A26086b29801fD79F4273E93770"
    ];
    // Initialize a 200 length array with Address Zero as initial values
    let mut unfunded_address_array: [&str; 200] = ["0x0000000000000000000000000000000000000000"; 200];
    // Array of Stream Cap of unfunded contracts . Initial value is 0.0 Ether
    let mut unfunded_cap_array:[f64;200] = [0.0;200];

    let mut  counter = 0;
    //Iterates to retrieve Ether balance , stream Cap , and current withdraw limit for each contract 
    for stream_address in address_array.iter_mut() {
        // Current stream contract address
        let address: Address = stream_address.parse()?;
        {
            // Init providers
            let    provider = Provider::<Http>::try_from(rpc_url)?;
            let  client_1 = Arc::new(provider);
            
            let client_2 = Arc::clone(&client_1);
            let  contract = STREAM_CONTRACT::new(address, client_2);
            let none= None;
            let client_3 = Arc::clone(&client_1);
            let ether = &client_3.get_balance(address,none).await?;// Get Ether balance 
           
            let  eth_balance = contract.cap().call().await?;// get Stream max cap 
            let stream_balance= contract.stream_balance().call().await?;// get current withdraw balance
            println!("{eth_balance} cap");
            // Need funding?
            if ether < &eth_balance {
                let check = Emoji("❌","[need funding]");
                println!("\n {} Need Funding!!\nETH_BALANCE: {}\nCAP: {} \nWithdraw: {stream_balance} ",check,ether,eth_balance);  
                unfunded_address_array[counter] = stream_address;
                let num: String = format_units(eth_balance, "ether").unwrap();
                println!("num{}",num);
                if num == "1.500000000000000000"  {
                    println!("lol");
                    (unfunded_cap_array[counter] = 1.5);
                }    
                else if num == "2.000000000000000000" {
                    println!("lol");

                     unfunded_cap_array[counter] = 2.0;
                 } 
                 else if num == "0.500000000000000000"{
                    println!("lol");

                    unfunded_cap_array[counter] = 0.5;
                 }
                else if num == "1.000000000000000000" {
                    println!("lol");

                    unfunded_cap_array[counter] = 1.0;
                }
                else{
                    println!("LOL");

                }
                println!("unfunded cap array{}",unfunded_cap_array[counter]);
               
                counter +=1;
            }
            // Don't need funding 
            else {  
                let check = Emoji("✅","[funded]");
                println!("\n {} Funded \nETH_BALANCE: {}\nCAP: {} \nWithdraw: {stream_balance} ",check,ether,eth_balance);  
            }
        }
    }
    // Copies the unfunded addresses and their respective cap . 
    // Highly inefficient way of doing this . 
    let mut n = 0 ;
        while unfunded_address_array[n] != "0x0000000000000000000000000000000000000000"{
            n+=1;
        }
        println!("Total No. of unfunded builders{}",n);
        if n==0 {

        }
        else if n==1{
            let output = format!(
                "{} {}",
                unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n==2{
            let output = format!(
                "{} {} \n{} {}",
                unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n==3{
            let output = format!(
                "{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==4{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==5{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==6{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==7{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==8{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==9{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==10{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==11{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==12{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==13{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==14{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==15{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==16{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==17{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==18{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==19{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==20{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-20],unfunded_cap_array[n-20],unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==21{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-21],unfunded_cap_array[n-21],unfunded_address_array[n-20],unfunded_cap_array[n-20],unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==22{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-22],unfunded_cap_array[n-22],unfunded_address_array[n-21],unfunded_cap_array[n-21],unfunded_address_array[n-20],unfunded_cap_array[n-20],unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==23{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-23],unfunded_cap_array[n-23],unfunded_address_array[n-22],unfunded_cap_array[n-22],unfunded_address_array[n-21],unfunded_cap_array[n-21],unfunded_address_array[n-20],unfunded_cap_array[n-20],unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
        else if n ==24{
            let output = format!(
                "{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} \n{} {} ",
                unfunded_address_array[n-24],unfunded_cap_array[n-24],unfunded_address_array[n-23],unfunded_cap_array[n-23],unfunded_address_array[n-22],unfunded_cap_array[n-22],unfunded_address_array[n-21],unfunded_cap_array[n-21],unfunded_address_array[n-20],unfunded_cap_array[n-20],unfunded_address_array[n-19],unfunded_cap_array[n-19],unfunded_address_array[n-18],unfunded_cap_array[n-18],unfunded_address_array[n-17],unfunded_cap_array[n-17],unfunded_address_array[n-16],unfunded_cap_array[n-16],unfunded_address_array[n-15],unfunded_cap_array[n-15],unfunded_address_array[n-14],unfunded_cap_array[n-14],unfunded_address_array[n-13],unfunded_cap_array[n-13],unfunded_address_array[n-12],unfunded_cap_array[n-12],unfunded_address_array[n-11],unfunded_cap_array[n-11],unfunded_address_array[n-10],unfunded_cap_array[n-10],unfunded_address_array[n-9],unfunded_cap_array[n-9],unfunded_address_array[n-8],unfunded_cap_array[n-8],unfunded_address_array[n-7],unfunded_cap_array[n-7],unfunded_address_array[n-6],unfunded_cap_array[n-6],unfunded_address_array[n-5],unfunded_cap_array[n-5],unfunded_address_array[n-4],unfunded_cap_array[n-4],unfunded_address_array[n-3],unfunded_cap_array[n-3],unfunded_address_array[n-2],unfunded_cap_array[n-2],unfunded_address_array[n-1],unfunded_cap_array[n-1]
            );
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(output).unwrap();
        }
    Ok(())
}



