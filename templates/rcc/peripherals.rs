{{!
// Add the following API for all the peripherals that are managed by the RCC:
//    - `rcc.enable_peripheral()` method,
//    - `rcc.disable_peripheral()` method,
//    - `rcc.reset_peripheral()` method,
//
// See the generated documentation for the exact list of provided methods.
~}}//! The RCC peripherals API

// FIXME use stm32ral::modify_reg;

impl super::RCC {
    {{~ #each buses }}
    /// Enable the {{peripheral}} peripheral clock.
    pub(crate) fn enable_{{peripheral}}(&mut self) {
        // FIXME modify_reg!(stm32ral::rcc, self.rcc, {{bus}}ENR, {{field}}EN: Enabled);
        self.rcc.{{bus}}ENR.write(self.rcc.{{bus}}ENR.read() | 1 << stm32ral::rcc::{{bus}}ENR::{{field}}EN::offset);
    }
    /// Disable the {{peripheral}} peripheral clock.
    pub(crate) fn disable_{{peripheral}}(&mut self) {
        // FIXME modify_reg!(stm32ral::rcc, self.rcc, {{bus}}ENR, {{field}}EN: Disabled);
        self.rcc.{{bus}}ENR.write(self.rcc.{{bus}}ENR.read() | 0 << stm32ral::rcc::{{bus}}ENR::{{field}}EN::offset);
    }
    /// Reset the {{peripheral}} peripheral.
    pub(crate) fn reset_{{peripheral}}(&mut self) {
        // FIXME modify_reg!(stm32ral::rcc, self.rcc, {{bus}}RSTR, {{field}}RST: Reset);
        // FIXME modify_reg!(stm32ral::rcc, self.rcc, {{bus}}RSTR, {{field}}RST: Clean);
        {{ #if resetable ~}}
        // FIXME Reset set the good bit but doesn't reset it.
        //self.rcc.{{bus}}RSTR.write(self.rcc.{{bus}}RSTR.read() | (1 << stm32ral::rcc::{{bus}}RSTR::{{field}}RST::offset));
        //self.rcc.{{bus}}RSTR.write(self.rcc.{{bus}}RSTR.read() | (0 << stm32ral::rcc::{{bus}}RSTR::{{field}}RST::offset));
        {{~ else ~}}
        compile_error!("{{peripheral}} reset is not supported");
        {{~ /if }}
    }
    {{~/ each }}
}
