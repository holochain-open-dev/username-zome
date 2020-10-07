import { Orchestrator } from '@holochain/tryorama'

const orchestrator = new Orchestrator()

require('./username')(orchestrator)

orchestrator.run()

