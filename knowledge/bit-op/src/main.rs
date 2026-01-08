use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct Permission:u8 {
        const READABLE = 4;
        const WRITABLE = 2;
        const EXECUTABLE = 1;
    }
}

impl Permission {
    fn to_u8(self) -> u8 {
        self.bits() as u8
    }
}
fn main() {
    let p = Permission::READABLE | Permission::WRITABLE | Permission::EXECUTABLE;

    if p & Permission::READABLE == Permission::READABLE {
        println!("【1】有读权限");
    } else {
        println!("【1】无读权限");
    }

    if p & Permission::WRITABLE == Permission::WRITABLE {
        println!("【1】有写权限");
    } else {
        println!("【1】无写权限");
    }

    if p & Permission::EXECUTABLE == Permission::EXECUTABLE {
        println!("【1】有执行权限");
    } else {
        println!("【1】无执行权限");
    }

    let p = p & !Permission::WRITABLE;

    if p & Permission::READABLE == Permission::READABLE {
        println!("【2】有读权限");
    } else {
        println!("【2】无读权限");
    }

    if p & Permission::WRITABLE == Permission::WRITABLE {
        println!("【2】有写权限");
    } else {
        println!("【2】无写权限");
    }

    if p & Permission::EXECUTABLE == Permission::EXECUTABLE {
        println!("【2】有执行权限");
    } else {
        println!("【2】无执行权限");
    }

    println!("【3】当前权限：{}, {:?}", p.to_u8(), p);
}
