use libimagstore::hook::Hook;
use libimagstore::hook::position::HookPosition;

struct CreateHook {
    position: HookPosition,
    config: Option<&Value>,
}

impl CreateHook {

    pub fn new(p: HookPosition) -> CreateHook {
        CreateHook {
            position: p,
        }
    }

}

impl Hook for CreateHook {

    fn name(&self) -> &'static str {
        "stdhook_git_create"
    }

    fn set_config(&mut self, config: &Value) {
        self.config = Some(config);
    }

}

impl StoreIdAccessor for CreateHook {

    fn access(&self, id: &StoreId) -> HookResult<()> {
        debug!("[GIT CREATE HOOK]: {:?}", id);
        Ok(())
    }

}

