use std::error::Error;

use pda6502v2emu::dbginfo::{self};

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let sample = r#"
version	major=2,minor=0
info	csym=0,file=10,lib=0,line=993,mod=10,scope=52,seg=9,span=710,sym=432,type=21
file	id=0,name="blinken.s",size=2677,mtime=0x63064D0E,mod=0
file	id=1,name="life.s",size=6455,mtime=0x633B69E8,mod=1
line	id=0,file=0,line=25,span=7
line	id=1,file=0,line=30,span=10
mod	id=0,name="blinken.o",file=0
mod	id=1,name="life.o",file=1
seg	id=4,name="ZEROPAGE",start=0x000000,size=0x0000,addrsize=zeropage,type=rw
seg	id=6,name="os",start=0x00F000,size=0x0758,addrsize=absolute,type=rw,oname="os.rom",ooffs=0
span	id=0,seg=6,start=0,size=1
span	id=1,seg=6,start=1,size=2
scope	id=0,name="",mod=0,size=95,span=50
scope	id=1,name="BlinkenStart",mod=0,type=scope,size=40,parent=0,sym=17,span=18
sym	id=17,name="BlinkenStart",addrsize=absolute,size=40,scope=0,def=38,ref=11,val=0xF000,seg=6,type=lab
sym	id=302,name="SidTunes",addrsize=absolute,size=1,scope=20,def=652,ref=123+642,val=0xF5CE,seg=6,type=lab
type	id=0,val="801120"
type	id=1,val="801A20"
"#;

    let info = dbginfo::parse(sample)?;

    assert_eq!(info.label(0xF000).unwrap(), "BlinkenStart");
    assert!(info.label(0x1234).is_none());

    assert_eq!(info.addr("SidTunes").unwrap(), 0xF5CE);
    assert!(info.addr("NopeNotHere").is_none());

    Ok(())
}
