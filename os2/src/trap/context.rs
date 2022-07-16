use riscv::register::sstatus::{self, Sstatus, SPP};

// the information when trap we need to keep 
#[repr(C)]
pub struct TrapContext {
    // we will save all register when we trap
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    // last i-addr 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
}
