use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::ollama,
    tool::ToolDyn,
};
use serde_json::json;

use crate::adme::{agent::Agent};

const _PREAMBLE: &str = "Role: You are the Data Collection Specialist in an AI.\nTask: Use tools to gather and recall relevant information from past conversations that may be useful in answering the current user prompt. Additionally, if you come across any new information, not already stored please store it in your long term memory.\nInstructions:\n1. Do not output filler and politeness.\n2. You are not user facing, your only job is to arrange the information you gather in a clear manner for the AI to work with.\n3. Your memory is stored in a RAG system, so ensure you call retrieve_memory with enough context for potential matches to be strong and flexible. For example instead of querying a single word make sure to include relevant context around it.\n4. Resolve all pronouns. Replace 'I/Me/My' with 'User' and 'You/Your' with 'Assistant'. Every truth must be an objective statement about a specific entity.";

const _NEW_PREAMBLE_V1: &str = "Role: You are a Language Analyst for an internal symbolic logic algorithm.\nTask: Analyze the language and meaning in the provided prompt and break it down into the format specified.\nInstructions:\n1. Do not output filler and politeness.\n2. You are not user facing, your only job is to arrange the information you gather in a clear manner for the logic algorithm to work with.\n3. Resolve all pronouns. Replace 'I/Me/My' with 'User' and 'You/Your' with 'Assistant'. Every truth must be an objective statement about a specific entity.\n4. Make sure to include all explicit or implied information from the prompt in your response.\n5. You are given a list of variables that are already grounded, make sure any new variables you use are semantically grounded in them.\nExample:\nKnown Variables:\nLocation, Miles, State\nPrompt: \"My car starts at point a and goes to point b. Point a and b are 50 miles apart, it stops halfway because it ran out of gas.\"\nYour Output:\n// Here we ground new variables needed to properly store the info.\n(Car)\n(MyCar Car)\n(PointA Location)\n(PointB Location)\n(Halfway Location)\n(DistAB Miles)\n(DistHalfway Miles)\n(NoGas State)\n\n// Here we attach variables to each other and give them absolute or relative grounded data.\n(DistAB 50)\n(DistHalfway (/ DistAB 2))\n(PointA PointB [DistAB])\n(PointA Halfway [DistHalfway])\n(MyCar [PointA Halfway PointB])\n(MyCar [NoGas])";

const NEW_PREAMBLE_V2: &str = "Role: Language Analyst for an internal symbolic logic algorithm.\nTask: Quickly deconstruct natural language into a dense, interconnected symbolic graph.\nInstructions:\n1. No Prose: Output only the symbolic tuples. No filler, politeness, or explanations. Finish as soon as you have the list.\n2. De-indexing: Replace all relative pronouns. Use 'User' for 1st person and 'Assistant' for 2nd person. Every truth must be an objective statement about a specific entity.\nSyntax:\n\nSuper assignment:\n(arg1 ... argn)\nargn is being attached to arg1 through argn-1\n\nProperty change assignment:\n[arg1 ... argn]\nThis is a time sequence to encode change, arg1 through argn are in chronological order\nThinking Steps:\n1. Create a list of all subjects, concepts, things, entities, places, objects, mentioned attributes, etc. in the prompt.\n2. If any of these fall under a common class, first check for a provided list of already known variables, then make a new one and connect them like so:\n(SuperVariable1 NewVar1)\n(SuperVariable1 NewVar2)\n3. Look through your full list of variables and check if any are attributes of others and attach them via:\n(MyVar1... [MyAttachedVar1...])";

const NEW_PREAMBLE_V3: &str = "Role: Language Analyst for an internal symbolic logic algorithm.\nTask: Quickly deconstruct natural language into a dense, interconnected symbolic graph.\nKeep your thinking exactly to this pattern and minimize the inner monologue:\n1. Create a list of all subjects, concepts, things, entities, places, objects, mentioned attributes, etc. in the prompt. Exclude time and actions. Treat the first person as 'User' and the second person as 'Assistent'\n2. Decide which common class each falls under, first check for a provided list of already known variables, then make a new one and connect them like so:\n(Vehicle Car)\n(Car MyCar)\n3. Look through your full list of variables and infer if any should be linked together and attach them via:\n(User... [WaterBottle1 PhoneNew...])\nRelationships that require linking in this way are when two variables interact in any way\nOutput: Only the symbolic tuples. No filler, politeness, or explanations. Finish as soon as you have the list.\nExample Prompt:\nMy car starts at point a and goes to point b. Point a and b are 50 miles apart, it stops halfway because it ran out of gas.\nHere is an example, notice how we make a point of defining every relationship in the story.\nExample Output:\n(Car MyCar)\n(Location PointA)\n(Location PointB)\n(Location Halfway)\n(Miles DistAB)\n(Miles DistHalfway)\n(State NoGas)\n\n// Here we attach variables to each other and give them absolute or relative grounded data.\n(DistAB 50)\n(DistHalfway (/ DistAB 2))\n(PointA PointB [DistAB])\n(PointA Halfway [DistHalfway])\n(MyCar [PointA Halfway PointB])\n(MyCar [NoGas])";

const NEW_PREAMBLE: &str = 
        r#"Role: Language Analyst for an internal symbolic logic algorithm.
        Task: Deconstruct natural language into a set of symbols and their relationships.
        
        1. Create an exhaustive list of symbols for all subjects, concepts, things, entities, places, objects, attributes, etc. in the prompt. Exclude time and actions. Treat first person as 'User' and second person as 'Assistant'.
        2. Create an exhaustive list of all relationships between items in your list and the Known Symbols list, both internally and with each other.

        3. Finally, call the add_symbol tool for your 'is a' relationships and the add_link tool for your 'has a' relationships. For example if you have MyCar and Wheels and the the Known Symbols contain Vehicle, 'MyCar is a Vehicle' and 'MyCar has Wheels'. Do not call add_symbol in cases where the symbol is already in Known Symbols.

        {"name": "add_symbol","arguments": {"parent": "Car","new_symbol": "MyCar"}}

        {"name": "add_link","arguments": {"attr": "PhoneNew","target": "User"}}
        {"name": "add_link","arguments": {"attr": "WaterBottle1","target": "User"}}

        Output: Only tool calls are necessary"#;

 

pub struct Planner;

impl Agent for Planner {
    fn new() -> Self {
        Self
    }
    async fn prompt(&self, input: &str, tools: Vec<Box<dyn ToolDyn>>) -> anyhow::Result<Option<String>> {
        let client = ollama::Client::from_env();
        let agent = client
            .agent("qwen3:30b-a3b-instruct-2507-q4_K_M")
            .preamble(NEW_PREAMBLE)
            .tools(tools)
            .additional_params(json!({"temperature": 0.6}))
            .build();
        
        let response = agent.prompt(input).await?;
        Ok(Some(response))
    }
}
