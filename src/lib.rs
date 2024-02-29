use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use spin_trigger::{cli::NoArgs, EitherInstance, TriggerAppEngine, TriggerExecutor};

wasmtime::component::bindgen!({
    world: "spin-test",
    path: "spin-test.wit",
    async: true
});

pub(crate) type RuntimeData = ();

pub struct TestTrigger {
    engine: TriggerAppEngine<Self>,
    test_components: Vec<Component>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestTriggerConfig {
    pub component: String,
}

#[derive(Clone, Debug)]
struct Component {
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TriggerMetadata {
    r#type: String,
}

#[async_trait]
impl TriggerExecutor for TestTrigger {
    const TRIGGER_TYPE: &'static str = "test";
    type RuntimeData = RuntimeData;
    type TriggerConfig = TestTriggerConfig;
    type RunConfig = NoArgs;

    async fn new(engine: TriggerAppEngine<Self>) -> Result<Self> {
        let test_components: Vec<_> = engine
            .trigger_configs()
            .map(|(_, config)| Component {
                id: config.component.clone(),
            })
            .collect();

        Ok(Self {
            engine,
            test_components,
        })
    }

    async fn run(self, _config: Self::RunConfig) -> Result<()> {
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            std::process::exit(0);
        });
        // let engine = Arc::new(self.engine);
        for component in self.test_components {
            let (instance, mut store) = self.engine.prepare_instance(&component.id).await?;
            let EitherInstance::Component(instance) = instance else {
                unreachable!()
            };
            let instance = SpinTest::new(&mut store, &instance)?;

            let res = instance.call_get_test_list(store).await?;
            match res {
                Ok(tests) => {
                    for test in tests {
                        let (instance, mut store) =
                            self.engine.prepare_instance(&component.id).await?;
                        let EitherInstance::Component(instance) = instance else {
                            unreachable!()
                        };
                        let instance = SpinTest::new(&mut store, &instance)?;
                        let res = instance.call_execute_test(store, &test).await;
                        match res {
                            Ok(val) => match val {
                                Ok(_) => println!("\"{test}\".... ok"),
                                Err(err) => println!("\"{test}\".... failed\n {err}"),
                            },
                            Err(e) => println!("Something went wrong: {e}"),
                        }
                    }
                }
                Err(_) => println!("ran into some error"),
            }
        }

        Ok(())
    }
}
