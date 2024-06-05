//use std::intrinsics::mir::Return;
//use std::iter::Sum;
use regex::Regex;
use std::{env, vec::Vec, fs, collections::hash_map::HashMap, io, rc::Rc, cell::RefCell, process, clone, u32, string};

pub trait InitTrait {
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self;
}

pub trait EvaluateTrait {
    fn evaluate(&self,sb: &mut SymbolTable, fb: &mut FuncTable, pt: &mut Playermap) -> NodeValue;
}

#[derive(Clone,Debug)]
pub enum NodeValue{
    Char(char),
    Int(i64),
    String(String),
    Unasigned,
    None,
}

#[derive(Clone, Debug)]
pub enum Nodes {
    BinOp(BinOp),
    UnOp(UnOp),
    IntVal(IntVal),
    StringVal(StringVal),
    UnasignedVal(UnasignedVal),
    NoOp(NoOp),
    Ident(Ident),
    Assign(Assign),
    Block(Block),
    Print(Print),
    Node(Node),
    While(While),
    If(If),
    Input(Input),
    FuncDec(FuncDec),
    FuncCall(FuncCall),
    VarDec(Ident),
    PlayerUse(PlayerActionNode),
    PlayerHit(PlayerActionNode),
    PlayerBlock(PlayerActionNode),
    PlayerState(PlayerActionNode),
    Wait(UnOp),
    Return(Return)
}

impl EvaluateTrait for Nodes{
    fn evaluate(&self,sb:&mut SymbolTable, ft:&mut FuncTable, pt:&mut Playermap) -> NodeValue {
        let result:NodeValue = match self{
            Nodes::BinOp(bin_op) => {
                //println!("eval binop");
                let mut ret_val : NodeValue = NodeValue::Int(0);
                let child1 : NodeValue = bin_op.n.children.get(0).unwrap().evaluate(sb,ft,pt);
                let child2 : NodeValue = bin_op.n.children.get(1).unwrap().evaluate(sb,ft,pt);
                match child1 {
                    NodeValue::Int(integer1) => {
                        match child2{
                            NodeValue::Int(integer2) => {
                             match &bin_op.n.value{
                                //operadores algébricos
                                NodeValue::Char(op) => {
                                    match op{
                                        '+' => {ret_val = NodeValue::Int(integer1 + integer2)},
                                        '-' => {ret_val = NodeValue::Int(integer1 - integer2)},
                                        '*' => {ret_val = NodeValue::Int(integer1 * integer2)},
                                        '/' => {ret_val = NodeValue::Int(integer1 / integer2)},
                                        '>' => {if integer1 < integer2 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}}
                                        '<' => {if integer1 > integer2 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}}
                                        _ => {println!("Operador não reconhecido")}
                                    }
                                }
                                //operadores realcionais
                                NodeValue::String(op) => {
                                    let op_string = op.clone();
                                    match op_string.as_str(){
                                        "and" => {if integer1 != 0 && integer2 != 0 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}},
                                        "or" => {if integer1 != 0 || integer2 != 0 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}},
                                        "==" => {if integer1 == integer2 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}},
                                        ".." => {let new_str = integer1.to_string() + integer2.to_string().as_str();return NodeValue::String(new_str);},
                                        x => {println!("Encontrou operador binário {x} não conhecido"); }
                                    }
                                }
                                _ => println!("Erro, número no BINOP")
                            }
                            },
                            NodeValue::String(string2) => {
                             match &bin_op.n.value{
                                //operadores algébricos
                                NodeValue::String(op) => {
                                    let op_string = op.clone();
                                    match op_string.as_str(){
                                        ".." => {let new_str = integer1.to_string() + string2.clone().as_str();return NodeValue::String(new_str);},
                                        _ => {println!("Operador não reconhecido")}
                                    }
                                },
                                _ => {}
                            }
                            }
                            _ => {println!("Erro, tipo errado dado à soma")}
                        }
                    },
                    NodeValue::String(string1) => {
                        match child2{
                            NodeValue::String(string2) => {
                                match &bin_op.n.value{
                                    NodeValue::String(op) =>{
                                        let op_string =op.clone();
                                        match op_string.as_str(){
                                            ".." => {let new_str = string1 + string2.clone().as_str();return NodeValue::String(new_str);},
                                            "==" => {if string2 == string1 {return NodeValue::Int(1)} return NodeValue::Int(0)}
                                            _ => {println!("Operação não encontrada para duas strings")}
                                        }
                                    },
                                    NodeValue::Char(op)=>{
                                        match op{
                                            '>' => {if string1 > string2{return NodeValue::Int(0)} return NodeValue::Int(1)}
                                            '<' => {if string1 < string2{return NodeValue::Int(0)} return NodeValue::Int(1)}
                                            _ => {}
                                        }
                                    }
                                    _ => {println!("Erro, não achou a operação dada para strings")}
                                }
                            },
                            NodeValue::Int(integer) => {
                                match &bin_op.n.value{
                                    NodeValue::String(op) =>{
                                        let op_string = op.clone();
                                        match op_string.as_str(){
                                            ".." => {let new_str = string1 + integer.to_string().as_str(); return NodeValue::String(new_str)},
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            _ => {println!("Erro, tentou concatenar string com algo que não era string")}
                        }
                    },
                    _ => {println!("Erro, tipo errado dado ao binop");}
                }
                ret_val
            },
            Nodes::UnOp(un_op) => {
                //println!("eval unnop");
                let mut ret_val : NodeValue = NodeValue::Int(0);
                let child1 : NodeValue = un_op.n.children.get(0).unwrap().evaluate(sb,ft,pt);
                match child1{
                    NodeValue::Int(integer) => {
                        match &un_op.n.value{
                        NodeValue::Char(op) => {
                            match op{
                                '+' => {ret_val = NodeValue::Int(integer)},
                                '-' => {ret_val = NodeValue::Int(-integer)},
                                _ => {println!("Operador não reconhecido no unop")}
                            }
                        }
                        NodeValue::String(op) => {
                            let op_clone = op.clone();
                            match op_clone.as_str(){
                                "not" => {if integer == 0 {return NodeValue::Int(1);} else {return NodeValue::Int(0);}},
                                _ => {println!("unop não encontrado"); return NodeValue::Int(0)}
                            }
                        }
                        _ => println!("Erro, número no UNOP")
                    }
                    }
                    _ => (println!("Erro, tipo errado fornecido ao UNOP"))
                }
                
               ret_val
            },
            Nodes::IntVal(int_val) => {
                //println!("eval intval");
                let ret_val : i64;
                match int_val.n.value{
                    NodeValue::Int(value) => {ret_val = value}
                    _ => {println!("Erro, operação no IntVal"); ret_val = 0}
                }
                NodeValue::Int(ret_val)
            },
            Nodes::StringVal(str_val) => {
                let ret_val : String;
                match &str_val.n.value{
                        NodeValue::String(value) => {ret_val = value.clone();},
                    _ => {println!("Erro, operação no StringVal"); ret_val = String::from("")}
                }
                NodeValue::String(ret_val)
            }
            Nodes::UnasignedVal(un_val) => {
                NodeValue::Unasigned
            }
            Nodes::Ident(ident) => {
                let mut var_value: NodeValue = NodeValue::Int(0);
                match &ident.n.value{
                    NodeValue::String(name) => {
                        //println!("Var name = {name}");
                        match sb.get_variable(name.clone()){
                            Some(variab) => {
                                match variab {
                                    Variables::Int(n) => {var_value = NodeValue::Int(n.clone())/*; println!("{n}")*/},
                                    Variables::String(n) => {var_value = NodeValue::String(n.clone())}
                                    Variables::Bool(n) => {println!("Bool não implementado");}
                                    _ => {println!("ERRO, tentou usar variável não iniciada")}
                                }
                            },
                            None => println!("Variavel {} usada antes de assignment", name)
                        }
                    },
                    _ => { println!("Identifier node has been given wrong type")},
                }
                var_value
            }
            Nodes::VarDec(var_dec) => {
                return NodeValue::Int(0);
            }
            Nodes::Assign(assign) => {
                let mut var_name: String = String::new();
                let var_name_child = assign.n.children.get(0).unwrap();
                match var_name_child{
                    Nodes::Ident(ident_node) => {
                        match &ident_node.n.value{
                            NodeValue::String(str) => { var_name = str.clone(); }
                            _ => { println!("Identifier node has been given wrong type"); }
                        }
                    }
                    x => {println!("Wrong node type:"); dbg!(x); return NodeValue::Int(1)}
                }
                let result_child = assign.n.children.get(1).unwrap();
                let result = result_child.evaluate(sb,ft,pt);
                //println!("var: {}",var_name);
                match result{
                    NodeValue::Int(n) => {sb.set_variable(var_name, Variables::Int(n));}
                    NodeValue::String(n) => {sb.set_variable(var_name, Variables::String(n))}
                    NodeValue::Unasigned => {sb.set_variable(var_name, Variables::Undeclared)}
                    _ => {println!("Assign desse tipo não foi implementado");}
                }
                NodeValue::Int(0)
            },
            Nodes::Block(block) => {

                //println!("eval block");
                match block.n.value.clone(){
                    NodeValue::String(block_name) => {println!("----------------- \n Begining of sequence {} \n-----------------",block_name)},
                    _ => {}
                }
                //let mut new_pt = Playermap::new();
                for child in block.n.children.iter(){
                    let is_return = false;
                    match child{
                        Nodes::Return(_) => {return child.evaluate(sb, ft, pt)},
                        _ => {}
                    }
                    child.evaluate(sb,ft, pt);
                }
                match block.n.value.clone(){
                    NodeValue::String(block_name) => {println!("-----------------\n End of sequence {}\n-----------------",block_name)},
                    _ => {}
                }
                return NodeValue::Int(0);
            },
            Nodes::Print(print) => {
                //println!("eval print");
                let child : NodeValue = print.n.children.get(0).unwrap().evaluate(sb,ft,pt);
                match child{
                    NodeValue::Int(val) => {println!("{}",val);},
                    NodeValue::String(val) => {println!("{}",val);},
                    NodeValue::Char(val) => {println!("{}",val);},
                    _ => {println!("Invalid operation for undeclared variable");}
                }
                return NodeValue::Int(0);
            },
            Nodes::If(if_node) =>{
                let condition = if_node.n.children.get(0).unwrap();
                let condition_result = condition.evaluate(sb,ft,pt);
                match condition_result {
                        NodeValue::Int(conditional_result) => {
                            if conditional_result == 1{
                                if_node.n.children.get(1).unwrap().evaluate(sb,ft,pt);
                                return NodeValue::Int(0)
                            }
                            if_node.n.children.get(2).unwrap().evaluate(sb,ft,pt) ;
                            return NodeValue::Int(0);
                    },
                    _ => {println!("Erro, tipo errado fornecido ao IF");}
                }
                return NodeValue::Int(0);
            },
            Nodes::While(while_node) => {
                let condition = while_node.n.children.get(0).unwrap();
                let block_node = while_node.n.children.get(1).unwrap();
                loop{
                    let result = condition.evaluate(sb,ft,pt);
                    match result {
                        NodeValue::Int(n) => {
                            if n == 0{break;}
                            else {block_node.evaluate(sb,ft,pt);}
                        }
                        _ => {println!("Erro, tipo errado dado pelo condicional do while"); break}
                    }
                }
                return NodeValue::Int(0)
            },
            Nodes::Input(input) => {
                //println!("pipi");
                let mut string_input = String::new();
                io::stdin().read_line(&mut string_input).unwrap();

                match string_input.trim().parse(){
                    Ok(n) => {return NodeValue::Int(n)}
                    Err(x) => {println!("Input inválido para int. Erro {x}"); return NodeValue::Int(0)}
                }
            },
            Nodes::FuncDec(func_dec) => {
                let func_name_node = &func_dec.n.value;
                let mut funct_name : String = String::new();
                match func_name_node{
                    NodeValue::String(fname) => funct_name = fname.clone(),
                    x => {println!("Wrong node type:"); dbg!(x); return NodeValue::Int(1)}
                }
                //println!("func dec:{}", funct_name);
                ft.set_function(funct_name, func_dec.clone());
                return NodeValue::Int(0)
            },
            Nodes::FuncCall(func_call) => {
                let func_name_thingy = &func_call.n.value;
                let func_name : String;
                match func_name_thingy{
                    NodeValue::String(func_name_thingy_val) => {func_name = func_name_thingy_val.clone();}
                    _ => {println!("Function didn't recieve proper value"); return NodeValue::Int(0)}
                }
                //println!("func call:{}", func_name);
                let mut func_node = ft.get_function(func_name).unwrap().clone();
                let mut func_sb: SymbolTable = SymbolTable { variables: HashMap::new() };
                let mut func_arguments : Vec<String> = Vec::new();
                for i in func_node.n.children.iter(){
                    match i {
                        Nodes::VarDec(var_dec) => {
                            let var_name = var_dec.n.value.clone();
                            match var_name{
                                NodeValue::String(var_name) => {
                                    func_arguments.push(var_name.clone());
                                    func_sb.set_variable(var_name.clone(), Variables::Undeclared);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                for i in 0..func_call.n.children.len(){
                    let eval_val = func_call.n.children[i].evaluate(sb, ft, pt);
                    match eval_val{
                        NodeValue::Int(num) => {func_sb.set_variable(func_arguments[i].clone(), Variables::Int(num));},
                        NodeValue::String(str_val) => {func_sb.set_variable(func_arguments[i].clone(), Variables::String(str_val));}
                        _ => {println!("Função não foi implementada para esse tipo de variável");}
                    }
                }
                let vec_of_children = func_node.n.children.clone();
                let func_block_node : &Nodes = vec_of_children.last().unwrap();
                return func_block_node.evaluate(&mut func_sb, ft, pt);
            }
            Nodes::Return(thing) => {
                let ret_val = thing.n.children[0].evaluate(sb, ft, pt);
                //dbg!(&thing.n.children[0]);
                //dbg!(ret_val.clone());
                return ret_val
            }
            Nodes::PlayerUse(player_action) => {
                let mut player = &mut Player{state: Playerstate::Idle, life: 0, current_delay :0};
                
                let inputs = player_action.n.children[0].evaluate(sb, ft, pt);
                let delay = player_action.n.children[1].evaluate(sb, ft, pt);

                let mut playername = String::new();
                match player_action.n.value.clone(){
                    NodeValue::String(x) => {
                        playername = x.clone();
                        match x.as_str(){
                            "PLAYER" => {
                                player = &mut pt.player;
                            },
                            "ENEMY" => {
                                player = &mut pt.enemy;
                            },
                            _ => {println!("playername not recognized during evaluate"); return NodeValue::Int(0)}
                        }
                    }
                    _ => {println!("Wrong value recieved during evaluate from player action");}
                }
                match player.state{
                    Playerstate::Idle => {},
                    _ => {println!("{playername} cannot attacks when not Idle."); return NodeValue::None }
                }
                match delay{
                    NodeValue::Int(delay) => {
                        player.current_delay = delay;
                    }
                    x => {dbg!(x);}
                }
                match inputs{
                    NodeValue::String(inp) => {println!("{playername} used {inp}", );}
                    _ => {}
                }

                player.state = Playerstate::Attacking;
                return  NodeValue::None;
            }
            Nodes::PlayerHit(player_action) => {
                let mut player = &mut Player{state: Playerstate::Idle, life: 0, current_delay :0};
                
                let inputs = player_action.n.children[0].evaluate(sb, ft, pt);
                let delay = player_action.n.children[1].evaluate(sb, ft, pt);
                let attack = player_action.n.children[2].evaluate(sb, ft, pt);

                let mut playername = String::new();
                match player_action.n.value.clone(){
                    NodeValue::String(x) => {
                        playername = x.clone();
                        match x.as_str(){
                            "PLAYER" => {
                                player = &mut pt.player;
                            },
                            "ENEMY" => {
                                player = &mut pt.enemy;
                            },
                            _ => {println!("playername not recognized during evaluate"); return NodeValue::Int(0)}
                        }
                    }
                    _ => {println!("Wrong value recieved during evaluate from player action");}
                }
                match delay{
                    NodeValue::Int(delay) => {
                        player.current_delay = delay;
                    }
                    x => {dbg!(x);}
                }
                match inputs{
                    NodeValue::String(inp) => {print!("{playername} was hit with {inp} ");}
                    _ => {}
                }
                match attack{
                    NodeValue::Int(dmg) => {
                        player.life = player.life - dmg;
                        println!("and took {} damage. {} life remains", dmg, player.life);
                        if player.life <= 0 {println!("O jogador {playername} perdeu")}
                    }
                    _ => {}
                }

                player.state = Playerstate::Hitstun;
                return  NodeValue::Int(0)
            }
            Nodes::PlayerBlock(player_action) => {
                let mut player = &mut Player{state: Playerstate::Idle, life: 0, current_delay :0};
                
                let inputs = player_action.n.children[0].evaluate(sb, ft, pt);
                let delay = player_action.n.children[1].evaluate(sb, ft, pt);

                let mut playername = String::new();
                match player_action.n.value.clone(){
                    NodeValue::String(x) => {
                        playername = x.clone();
                        match x.as_str(){
                            "PLAYER" => {
                                player = &mut pt.player;
                            },
                            "ENEMY" => {
                                player = &mut pt.enemy;
                            },
                            _ => {println!("playername not recognized during evaluate"); return NodeValue::Int(0)}
                        }
                    }
                    _ => {println!("Wrong value recieved during evaluate from player action");}
                }
                match delay{
                    NodeValue::Int(delay) => {
                        player.current_delay = delay;
                    }
                    x => {dbg!(x);}
                }
                match inputs{
                    NodeValue::String(inp) => {println!("{playername} blocked {inp} ");}
                    _ => {}
                }

                player.state = Playerstate::Blockstun;
                return  NodeValue::Int(0)
            }
            Nodes::PlayerState(player_action) => {
                let mut player = &mut Player{state: Playerstate::Idle, life: 0, current_delay :0};

                let mut playername = String::new();
                let state_to_check = match player_action.n.children[0].evaluate(sb, ft, pt){
                    NodeValue::String(str_val) => {
                        match str_val.as_str(){
                            "IDLE" => {Playerstate::Idle}
                            "BLOCKSTUN" => {Playerstate::Blockstun},
                            "HITSTUN" => {Playerstate::Hitstun},
                            "GROUNDED" => {Playerstate::Grounded},
                            "ATTACKING" => {Playerstate::Attacking},
                            "JUMPING" => {Playerstate::Jumping},
                            _ => {Playerstate::Idle}
                        }
                    },
                    _ => {Playerstate::Idle}
                };

                match player_action.n.value.clone(){
                    NodeValue::String(x) => {
                        playername = x.clone();
                        match x.as_str(){
                            "PLAYER" => {
                                player = &mut pt.player;
                            },
                            "ENEMY" => {
                                player = &mut pt.enemy;
                            },
                            _ => {println!("playername not recognized during evaluate"); return NodeValue::Int(0)}
                        }
                    }
                    _ => {println!("Wrong value recieved during evaluate from player action");}
                };

                let is_same = if state_to_check == player.state { 1 } else { 0 };
                return NodeValue::Int(is_same);
            }
            Nodes::Wait(thingy) => {
                let thingmabob = thingy.n.children[0].evaluate(sb, ft, pt);
                match thingmabob{
                    NodeValue::Int(delay) => {
                        pt.player.current_delay -= delay;
                        pt.enemy.current_delay -= delay;
                        //pt.update_states();
                    }
                    _ => {}
                };
                println!("Gamestates after wait: player: {:>+} | enemy: {:>+}", -pt.player.current_delay, -pt.enemy.current_delay);
                pt.update_states();
                return NodeValue::None;
            }
            Nodes::NoOp(_) => {return NodeValue::Int(0);},
            _ => {println!("Node avaliado não foi implementado"); NodeValue::Int(0)}

        };
    return result;
    }
}

enum Variables{
    Int(i64),
    String(String),
    Input(String),
    Delay(i64),
    Bool(bool),
    Undeclared
}


#[derive(Clone,Debug)]
pub struct Node{
    value: NodeValue,
    children: Vec<Nodes>,
}

#[derive(Clone,Debug)]
pub struct Input{
    n: Node,
}

impl InitTrait for Input{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao Input");
        }
        Input {n:Node{value: value, children:children}}   
    }
}


#[derive(Clone,Debug)]
pub struct PlayerActionNode{
    n: Node,
}

impl InitTrait for PlayerActionNode{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() < 1{
            println!("Numero errado de filhos passado ao PlayerActionNode");
        }
        match value.clone(){
            NodeValue::String(x) => {},
            _ => {println!("Valor inválido fornecido ao PlayerActionNode");}
        }
        PlayerActionNode {n:Node{value: value, children:children}}   
    }
}


#[derive(Clone,Debug)]
pub struct While{
    n : Node,
}

impl InitTrait for While{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 2{
            println!("Numero errado de filhos passado ao While");
        }
        While {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct If{
    n : Node,
}

impl InitTrait for If{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 3{
            println!("Numero errado de filhos passado ao If");
        }
        If {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct Ident{
    n: Node,
}

impl InitTrait for Ident{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao Ident");
        }
        Ident {n:Node{value: value, children:children}}   
    }
}

#[derive(Clone,Debug)]
pub struct Assign{
    n : Node
}

impl InitTrait for Assign{
    fn new(value:NodeValue, children: Vec<Nodes>) -> Self{
        //println!("Criado o bloco assign");
        if children.len() != 2{
            println!("Numero errado de filhos passado ao ASSIGN");
        }
        Assign {n:Node{value: value, children:children}}
        
    }
    
}

#[derive(Clone,Debug)]
pub struct Block{
    n: Node,
}


impl InitTrait for Block{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Block{
        Block {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct Print{
    n: Node,
}

impl InitTrait for Print{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Print{
        Print {n:Node{value: value, children:children}}
    }
}


#[derive(Clone,Debug)]
pub struct BinOp{
    n : Node,
}

impl InitTrait for BinOp{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 2{
            println!("Numero errado de filhos passado ao BINOP");
        }
        BinOp {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct UnOp{
    n : Node,
}

impl InitTrait for UnOp{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 1{
            println!("Numero errado de filhos passado ao UNOP");
        }
        UnOp {n:Node{value: value, children:children}}
    }
        
}


#[derive(Clone,Debug)]
pub struct UnasignedVal{
    n : Node,
}

impl InitTrait for UnasignedVal{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao IntVal");
        }
        Self {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct IntVal{
    n : Node,
}

impl InitTrait for IntVal{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao IntVal");
        }
        Self {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct StringVal{
    n : Node,
}

impl InitTrait for StringVal{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao IntVal");
        }
        Self {n:Node{value: value, children:children}}
    }
}

#[derive(Clone,Debug)]
pub struct FuncDec{
    n : Node,
}

impl InitTrait for FuncDec{
    fn new(value: NodeValue, children : Vec<Nodes>) -> Self{
        if children.len() < 1{
            println!("Numero errado de filhos passado ao FuncDec");
        }
        Self {n:Node{value:value, children: children}}
    }
}

#[derive(Clone,Debug)]
pub struct FuncCall{
    n : Node,
}

impl InitTrait for FuncCall{
    fn new(value: NodeValue, children : Vec<Nodes>) -> Self{
        Self {n:Node{value:value, children: children}}
    }
}

#[derive(Clone,Debug)]
pub struct Return{
    n : Node,
}

impl InitTrait for Return{
    fn new(value: NodeValue, children : Vec<Nodes>) -> Self{
        Self {n:Node{value:value, children: children}}
    }
}

#[derive(Clone,Debug)]
pub struct NoOp{
    //n : Node,
}

impl InitTrait for NoOp{
    fn new(value: NodeValue, children: Vec<Nodes>) -> Self{
        if children.len() != 0{
            println!("Numero errado de filhos passado ao NoOp");
            match value {
                NodeValue::Int(n) => {println!("{}",n);}
                _ => {}
            }
        }
        //Self {n:Node{value: value, children:children}}
        Self{}
    }
}


fn pre_pro_filter(unprocessed_string:String) -> String{
    let mut processed_string = String::new();
    for line in unprocessed_string.lines(){
        let mut gambiarra = 0;
        for clean_line in line.split("--"){
            if gambiarra == 0{
                processed_string += clean_line;
                processed_string += "\n";
                gambiarra += 1;
            }
        }
    }
    return processed_string;

}


pub struct SymbolTable{
    variables: HashMap<String, Variables>,
}

impl SymbolTable{
    fn set_variable(&mut self, variable_name : String, variable_value : Variables){
        self.variables.insert(variable_name, variable_value);
    }
    fn get_variable(&self, variable_name : String) -> Option<&Variables>{
        self.variables.get(&variable_name)
    }
}

#[derive(Clone,Debug, PartialEq)]
pub enum Playerstate{
    Attacking,
    Hitstun,
    Blockstun,
    Idle,
    Jumping,
    Grounded
}

#[derive(Clone,Debug)]
pub struct Player{
    state: Playerstate,
    current_delay: i64,
    life: i64
}

#[derive(Clone,Debug)]
pub struct Playermap{
    player : Player,
    enemy : Player
}

impl Playermap{
    fn new() -> Playermap{
        let player = Player { state: Playerstate::Idle, current_delay: 0, life: 100 };
        let enemy = Player { state: Playerstate::Idle, current_delay: 0, life: 100 };
        Playermap { player: player, enemy: enemy }
    }
    fn update_states(&mut self){
        let gambirra : Vec<&mut Player> = vec![&mut self.player, &mut self.enemy];
        for i in gambirra{
            if i.current_delay <= 0{
                i.current_delay = 0;
                i.state = Playerstate::Idle;
            }
        }
    }
}

pub struct FuncTable{
    functions: HashMap<String, FuncDec>
}

impl FuncTable{
    fn set_function(&mut self, func_name : String, func_node : FuncDec){
        self.functions.insert(func_name, func_node);
    }
    fn get_function(&self, func_name : String) -> Option<&FuncDec>{
        self.functions.get(&func_name)
    }
}

struct Token {
    token: String,
    value: i64,
}

impl Token{
    fn new() -> Self{
        Token {token:String::from("0"), value:0}
    }
    fn blank(&mut self){
        self.token = String::from("");
        self.value = 0;
    }
}

struct Tokenizer {
    source: String,
    position: usize,
    next: Token,
}

impl Tokenizer {
    fn new(source_string: String) -> Self{
        let my_token: Token = Token::new();
        Self {source: source_string, position:0,next: my_token}
    }
    fn select_next(&mut self) {
        
        //não tava com saco de fazer o tokenizer sem regex pra capturar input, então cá está
        let inputs_regex : Regex = Regex::new(r"^([a-d]|u|d|l|r|(L|M|H)(P|K)|←|→|↑|↓|H|P|K|S|HS|D|[0-9]|\+])+$").unwrap();

        if self.position < self.source.len(){
            let mut current_char = self.source.chars().nth(self.position).unwrap();
            //tratando espaco
            while (current_char == ' ' || current_char == '\t') && self.position < self.source.chars().count(){
                current_char = self.source.chars().nth(self.position).unwrap();
                if current_char != ' ' && current_char != '\t'{
                    break;
                }
                self.position += 1;
            }

            //tratando operadores
            if ['+','-','*','/', '(', ')', '<', '>', ':'].contains(&current_char){
                self.next.blank();
                self.next.value = -1;
                self.next.token = String::from(current_char);
                self.position += 1;
            }

           //tratando strings
            else if current_char == '\"'{
                self.position += 1;
                self.next.blank();
                self.next.value = -5;
                current_char = self.source.chars().nth(self.position).unwrap();
                while current_char != '\"' {
                    self.next.token.push(current_char);
                    //print!("c: {current_char}");
                    self.position += 1;
                    current_char = self.source.chars().nth(self.position).unwrap();
                }
                self.position += 1;
            }



            //tratando números
            else if current_char.is_digit(10){
                self.next.blank();
                while current_char.is_digit(10){
                    self.next.token.push(current_char);
                    match self.next.token.parse(){
                        Ok(n) => self.next.value = n,
                        Err(_) => {} //println!("Deu ruim! char {}", self.next.token)
                    }
                    self.position += 1;
                    match self.source.chars().nth(self.position){
                        Some(n) => current_char = n,
                        None => break
                    }
                }
                match self.source.chars().nth(self.position){
                    Some(n) => current_char = n,
                    None => {}
                }
                //check if a delay was invoked
                if current_char == 'f'{
                    self.position += 1;
                }
                else if ['P','K','S','H'].contains(&current_char){
                    self.next.value = -20;
                    let mut inputs = self.next.token.clone();
                    inputs.push(current_char);
                    while inputs_regex.is_match(inputs.as_str()){
                        self.next.token = inputs.clone();
                        self.position += 1;
                        match self.source.chars().nth(self.position){
                            Some(n) => current_char = n,
                            None => {}
                        }
                        inputs.push(current_char);
                    }
                }
            }

            //tratando identifiers
            else if current_char.is_alphabetic() || ['_'].contains(&current_char) {
                self.next.blank();
                self.next.value = -2; //codigo para identifier
                while (current_char.is_alphanumeric() || current_char == '_') && current_char != '\n'{
                    self.next.token.push(current_char);
                    self.position += 1;
                    match self.source.chars().nth(self.position){
                        Some(char) => current_char = char,
                        None => break
                    }
                    if current_char == '\n' || current_char == '\r'{
                        break
                    }
                } if inputs_regex.is_match(self.next.token.clone().as_str()){
                    self.next.value = -20;
                }
                //println!("{}", self.next.token.as_str());
            }

            //tratando assignment
            else if current_char == '='{
                self.next.blank();
                self.next.value = -3;
                self.next.token = String::from(current_char);
                self.position +=1;
                match self.source.chars().nth(self.position){
                    Some('=') => {self.next.token = String::from("=="); self.position += 1;},
                    Some(_) => {},
                    None => {}
                }
            }

            else if current_char == '.'{
                self.next.blank();
                self.next.token = String::from(current_char);
                self.position +=1;
                self.next.value = -2;
                match self.source.chars().nth(self.position){
                    Some('.') => {self.next.token = String::from(".."); self.position += 1;},
                    Some(_) => {},
                    None => {}
                }
            }

            else if current_char == ','{
                self.next.blank();
                self.next.token = String::from(current_char);
                self.position +=1;
                self.next.value = -2;
            }

            else if current_char == '\n' || current_char == '\r'{
                self.next.blank();
                self.next.value = -4;
                self.next.token = String::from(current_char);
                self.next.token.push(current_char);
                self.position +=1;
            }

            //tratando linebreak
            else{
                self.position += 1;
                if current_char != ' '{
                    println!("Encontrou símbolo insperádo {}", current_char);
                }
            }
        }
        else{
            self.next.value = - 10; //identifica que acabou o arquivo 
            println!("tentou select next após fim do arquivo");
            process::exit(1);
        }
        return   
    }
}


struct Parser<'a> {
    tokenizer : Tokenizer,
    symbol_table : &'a SymbolTable,
}

impl<'a> Parser<'a> {
    fn new(string_to_parse: String, symbol_table : &'a SymbolTable) -> Self{
        Self {tokenizer: Tokenizer::new(string_to_parse), symbol_table: symbol_table }
    }
    fn statement(&mut self) -> Nodes{
        let mut top_node: Nodes = Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]));
        //println!("statement token: {}", self.tokenizer.next.token.clone());
        if self.tokenizer.next.value == -4{
            self.tokenizer.select_next();
            return top_node
        }

        match self.tokenizer.next.value {
            -2 => {
                /* 
                if self.tokenizer.next.token.as_str() == "local"{
                    self.tokenizer.select_next();
                    let ident_node = Ident::new(NodeValue::String(self.tokenizer.next.token.clone()), vec![]);
                    //println!("ident node for:{}", &self.tokenizer.next.token);
                    self.tokenizer.select_next();
                    if self.tokenizer.next.token.as_str() == "="{
                        self.tokenizer.select_next();
                        let left_node = self.boolean_expression();
                        self.tokenizer.select_next();
                        return Nodes::Assign(Assign::new(NodeValue::Char('='), vec![Nodes::Ident(ident_node), left_node]));
                    }
                    let left_node =  UnasignedVal::new(NodeValue::Unasigned, vec![]);
                    return Nodes::Assign(Assign::new(NodeValue::Char('='), vec![Nodes::Ident(ident_node), Nodes::UnasignedVal(left_node)]))
                }
                */
                //
                if self.tokenizer.next.token.as_str() == "PLAYER" || self.tokenizer.next.token.as_str() == "ENEMY"{
                    let actor_player = self.tokenizer.next.token.clone();
                    self.tokenizer.select_next();
                    let player_action : Nodes;
                    if self.tokenizer.next.token.as_str() == "uses"{
                        self.tokenizer.select_next();
                        let attack_input = self.boolean_expression();
                        self.tokenizer.select_next();
                        let attack_delay = self.boolean_expression();
                        player_action = Nodes::PlayerUse(PlayerActionNode::new(NodeValue::String(actor_player), vec![attack_input, attack_delay]));
                        return player_action
                    }
                    else if self.tokenizer.next.token.as_str() == "hit"{
                        self.tokenizer.select_next();
                        if self.tokenizer.next.token.as_str() != "with"{
                            println!("Erro, não encontrou verbo correto após hit. Adicione o verbo \"with\"");
                        }
                        self.tokenizer.select_next();
                        let attack_input = self.boolean_expression();
                        self.tokenizer.select_next();
                        let attack_delay = self.boolean_expression();
                        self.tokenizer.select_next();
                        let attack_damage = self.boolean_expression();
                        player_action = Nodes::PlayerHit(PlayerActionNode::new(NodeValue::String(actor_player), vec![attack_input, attack_delay, attack_damage]));
                        return player_action
                    }
                    else if self.tokenizer.next.token.as_str() == "blocks"{
                        self.tokenizer.select_next();
                        let attack_input = self.boolean_expression();
                        self.tokenizer.select_next();
                        let attack_delay = self.boolean_expression();
                        player_action = Nodes::PlayerBlock(PlayerActionNode::new(NodeValue::String(actor_player), vec![attack_input, attack_delay]));
                        return player_action
                    }
                }
                if self.tokenizer.next.token.as_str() == "wait" {
                    self.tokenizer.select_next();

                    let wait_node = Nodes::Wait(UnOp::new(NodeValue::Char(' '), vec![self.boolean_expression()]));
                    return wait_node;
                }
                if self.tokenizer.next.token.as_str() == "print"{
                    self.tokenizer.select_next();
                    top_node = Nodes::Print(Print::new(NodeValue::Int(0), vec![self.boolean_expression()]));
                    //self.tokenizer.select_next();
                    //println!("end of print char:{}",self.tokenizer.next.token.clone());
                    //println!("end of print val:{}",self.tokenizer.next.value);
                    return top_node;
                }
                else if self.tokenizer.next.token.as_str() == "function"{
                    //println!("entrou na funct");
                    self.tokenizer.select_next();
                    if self.tokenizer.next.value != -2{
                        println!("Encontrou algo errado depois de function");
                    }
                    let function_name = self.tokenizer.next.token.clone();
                    self.tokenizer.select_next();
                    if self.tokenizer.next.token != "("{
                        println!("Não encontrou abertura de parenteses na declaração de função");
                    }
                    let mut arguments : Vec<Nodes> = vec![];
                    self.tokenizer.select_next();
                    while self.tokenizer.next.token != ")"{
                        arguments.push(Nodes::VarDec(Ident::new(NodeValue::String(self.tokenizer.next.token.clone()) , vec![])));
                        self.tokenizer.select_next();
                        if self.tokenizer.next.token.as_str() == ","{
                            self.tokenizer.select_next();
                        }
                        else if self.tokenizer.next.token.as_str() != ")"{
                            println!("encontrou valor inesperado na declaração de função{}", self.tokenizer.next.token);
                            process::exit(1);
                        }
                    }
                    self.tokenizer.select_next();
                    let mut block_root = Block::new(NodeValue::Char(' '), vec![]);
                    while self.tokenizer.next.token != "end"{
                        //println!("{}",self.tokenizer.next.token.clone());
                        block_root.n.children.push(self.statement());
                    }
                    arguments.push(Nodes::Block(block_root));
                    self.tokenizer.select_next();
                    //println!("fim da funct{}",self.tokenizer.next.token);
                    return Nodes::FuncDec(FuncDec::new(NodeValue::String(function_name), arguments))

                }
                else if self.tokenizer.next.token.as_str() == "return"{
                    self.tokenizer.select_next();
                    //println!("caiu no ret");
                    let a = Nodes::Return(Return::new(NodeValue::Int(0), vec![self.boolean_expression()]));
                    //dbg!(&a);
                    return a
                }
                else if self.tokenizer.next.token.as_str() == "while"{
                    self.tokenizer.select_next();
                    if self.tokenizer.next.token != ":"{
                        println!("Faltou dois pontos após criação do while");
                    }
                    self.tokenizer.select_next();
                    let eval_expr = self.boolean_expression();
                    /*
                    if self.tokenizer.next.token.as_str() != "do"{
                        println!("Faltando DO após while");
                        return Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]))
                    }
                    */
                    if self.tokenizer.next.value != -4{
                        println!("Valor diferente de newline encontrado após condição do while : {}", self.tokenizer.next.token.clone());
                        return Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]))
                    }
                    let mut while_block_children: Vec<Nodes> = vec![];
                    self.tokenizer.select_next();
                    loop {
                        if self.tokenizer.next.token.as_str() == "end"{
                            self.tokenizer.select_next();
                            break;
                        }
                        if self.tokenizer.next.value == -10{
                            println!("Bloco While não foi fechado, EOF encontrado antes do end");
                            break;
                        }
                        while_block_children.push(self.statement());
                    }
                    let while_block = Nodes::Block(Block::new(NodeValue::Int(0), while_block_children));
                    let while_node = While::new(NodeValue::Int(0), vec![eval_expr, while_block]);
                    return Nodes::While(while_node);
                }
                else if self.tokenizer.next.token.as_str() == "if"{
                    self.tokenizer.select_next();
                    if self.tokenizer.next.token != ":"{
                        println!("Faltando then após if");
                    }
                    self.tokenizer.select_next();
                    let condition = self.boolean_expression();

                    if self.tokenizer.next.value != -4{
                        println!("Valor diferente de newline encontrado após condição no if: {}", self.tokenizer.next.token.clone());
                        return Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]))
                    }
                    let mut if_block_children: Vec<Nodes> = vec![];
                    self.tokenizer.select_next();
                    loop {
                        if self.tokenizer.next.token.as_str() == "end" || self.tokenizer.next.token.as_str() == "else"{
                            break;
                        }
                        if self.tokenizer.next.value == -10{
                            println!("Bloco If não foi fechado, EOF encontrado antes do end");
                            break;
                        }
                        if_block_children.push(self.statement());
                    }
                    let mut else_block_children: Vec<Nodes> = vec![];
                    if self.tokenizer.next.token.as_str() == "else"{
                        self.tokenizer.select_next();
                        loop {
                            if self.tokenizer.next.token.as_str() == "end"{
                                self.tokenizer.select_next();
                                break;
                            }
                            if self.tokenizer.next.value == -10{
                                println!("Bloco else não foi fechado, EOF encontrado antes do end");
                                break;
                            }
                            else_block_children.push(self.statement());
                        }
                    }
                    self.tokenizer.select_next();
                    let if_block = Nodes::Block(Block::new(NodeValue::Int(0), if_block_children));
                    let else_block = Nodes::Block(Block::new(NodeValue::Int(0), else_block_children));
                    return Nodes::If(If::new(NodeValue::Int(0), vec![condition, if_block, else_block]))

                }
                else{
                    //println!("Token: {}", self.tokenizer.next.token.clone());
                    let ident_name = self.tokenizer.next.token.clone();
                    self.tokenizer.select_next();
                    if self.tokenizer.next.token.as_str() == "("{
                        self.tokenizer.select_next();
                        let mut child = Vec::new();
                        while self.tokenizer.next.token.as_str() != ")"{
                            //println!("thingy st:{}", self.tokenizer.next.token);
                            let val = self.boolean_expression();
                            child.push(val);
                            //self.tokenizer.select_next();
                            if self.tokenizer.next.token.as_str() == ","{
                                self.tokenizer.select_next();
                            }
                            else if self.tokenizer.next.token.as_str() != ")"{
                                println!("encontrou valor inesperado na chamada de função statement {}", self.tokenizer.next.token);
                                process::exit(1);
                            }
                        }
                        self.tokenizer.select_next();
                        //println!("fl:{}", self.tokenizer.next.token);
                        return Nodes::FuncCall(FuncCall::new(NodeValue::String(ident_name), child));
                    }
                    if self.tokenizer.next.token.as_str() != "="{
                        println!("Identifier {} mal declarado", self.tokenizer.next.token.as_str())
                    }
                    let ident_node = Ident::new(NodeValue::String(ident_name.clone()), vec![]);
                    self.tokenizer.select_next();
                    let left_node = self.boolean_expression();
                    self.tokenizer.select_next();
                    //println!("assign to variable:{}",ident_name.clone());
                    return Nodes::Assign(Assign::new(NodeValue::Char('='), vec![Nodes::Ident(ident_node), left_node]));
                }
            },
            -4 => return top_node,
            x=> {println!("Statement recebeu argumento errado {x}, valor: {}", self.tokenizer.next.token);}
    //
        }
        self.tokenizer.select_next();
        if self.tokenizer.next.value == -4{
            return top_node
        }
        else {
            return Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]));
        }
    }

    fn boolean_expression(&mut self) -> Nodes{
        let mut top_node = self.boolean_term();
        loop {
            //println!("Token soma 2: {}", self.tokenizer.next.token.clone());
            match self.tokenizer.next.token.as_str() {
                "or" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.boolean_term();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::String(String::from("or")), vec![left_node, right_node]));
                },
                ".." => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.rel_expression();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::String(String::from("..")), vec![left_node, right_node]));
                },
                _ => break,
            }
        }
        return top_node;
    }

    fn boolean_term(&mut self) -> Nodes{
        let mut top_node = self.rel_expression();
        loop {
            //println!("Token soma 2: {}", self.tokenizer.next.token.clone());
            match self.tokenizer.next.token.as_str() {
                "and" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.rel_expression();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::String(String::from("and")), vec![left_node, right_node]));
                }
                _ => break,
            }
        }
        return top_node;
    }

    fn rel_expression(&mut self) -> Nodes{
        let mut top_node = self.parse_expression();
        loop {
            match self.tokenizer.next.token.as_str() {
                "==" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.parse_expression();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::String(String::from("==")), vec![left_node, right_node]));
                },
                ">" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.parse_expression();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('<'), vec![left_node, right_node]));
                },
                "<" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.parse_expression();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('>'), vec![left_node, right_node]));
                }
                _ => break,
            }
        }
        return top_node;
        
    }

    fn check_value(&mut self, val : String) -> bool{
        if self.tokenizer.next.token == val{
            return true;
        } else {
            println!("Não encontrou o valor {val}, {} encontrado no lugar!", self.tokenizer.next.token);
            return false;
        }
    }

    fn block(&mut self) -> Nodes{
        self.check_value(String::from("begin"));
        self.tokenizer.select_next();
        self.check_value(String::from(":"));
        self.tokenizer.select_next();
        let block_name = self.tokenizer.next.token.clone();
        self.tokenizer.select_next();
        let mut block_root = Block::new(NodeValue::String(block_name), vec![]);
        while self.tokenizer.next.token != "end"{
            block_root.n.children.push(self.statement());
        }
        self.tokenizer.select_next();
        return Nodes::Block(block_root);
    }

    fn program(&mut self) -> Nodes{
        let mut block_root = Block::new(NodeValue::None, vec![]);
        while self.tokenizer.position < self.tokenizer.source.len(){
            if self.tokenizer.next.value != -4{
                block_root.n.children.push(self.block());
            }
            else{
                self.tokenizer.select_next();
            }
        }
        return Nodes::Block(block_root);
    }

    fn factor(&mut self) -> Nodes{
        let number : i64;
        //println!("factor: {}", self.tokenizer.next.token.as_str());
        if self.tokenizer.next.value >= 0{ //não é operador
            number = self.tokenizer.next.value;
            self.tokenizer.select_next();
            let intval_node: IntVal = IntVal::new(NodeValue::Int(number),vec![]);
            return Nodes::IntVal(intval_node)
        }
        else if self.tokenizer.next.value == -5{
            //self.tokenizer.select_next();
            let string : String = self.tokenizer.next.token.clone();
            self.tokenizer.select_next();
            let stringval_node = StringVal::new(NodeValue::String(string), vec![]);
            return Nodes::StringVal(stringval_node)
        }
        else if self.tokenizer.next.value == -2 { //É um identifier
            if self.tokenizer.next.token.as_str() == "PLAYER" || self.tokenizer.next.token.as_str() == "ENEMY"{
                let playername = self.tokenizer.next.token.clone();
                self.tokenizer.select_next();
                if self.tokenizer.next.token.as_str() != "in"{
                    println!("Erro no código, faltou In depois do {playername}");
                }
                self.tokenizer.select_next();
                let playerstate = self.tokenizer.next.token.clone();
                self.tokenizer.select_next();
                return Nodes::PlayerState(PlayerActionNode::new(NodeValue::String(playername), vec![Nodes::StringVal(StringVal::new(NodeValue::String(playerstate), vec![]))]));

            }
            if self.tokenizer.next.token.as_str() == "read"{
                self.tokenizer.select_next();
                if self.tokenizer.next.token.as_str() != "("{ println!("parenteses do read não encontrado");}
                self.tokenizer.select_next();
                if self.tokenizer.next.token.as_str() != ")"{ println!("parenteses do read não fechado");}
                self.tokenizer.select_next();
                let read_node = Input::new(NodeValue::Int(0), vec![]);
                return Nodes::Input(read_node);
            }
            else if self.tokenizer.next.token.as_str() == "not"{
                self.tokenizer.select_next();
                let factor_rec_node = self.factor();
                let operation_node: UnOp = UnOp::new(NodeValue::String(String::from("not")), vec![factor_rec_node]);
                return Nodes::UnOp(operation_node)
            }
            //println!("ident no lugar certo");
            let name = self.tokenizer.next.token.clone();
            self.tokenizer.select_next();

            if self.tokenizer.next.token.as_str() == "("{
                self.tokenizer.select_next();
                let mut child = Vec::new();
                while self.tokenizer.next.token.as_str() != ")"{
                    //println!("thing:{}",self.tokenizer.next.token.clone());
                    let val = self.boolean_expression();
                    child.push(val);
                    //self.tokenizer.select_next();
                    if self.tokenizer.next.token.as_str() == ","{
                        self.tokenizer.select_next();
                    }
                    else if self.tokenizer.next.token.as_str() != ")"{
                        println!("encontrou valor inesperado na chamada de função {}", self.tokenizer.next.token.clone());
                        process::exit(1);
                    }
                    }
                self.tokenizer.select_next();
                //println!("ended func:{}", name);
                return Nodes::FuncCall(FuncCall::new(NodeValue::String(name), child));
            }
            return Nodes::Ident(Ident::new(NodeValue::String(name), vec![]))
        }
        else if self.tokenizer.next.value == -20{
            return Nodes::StringVal(StringVal::new(NodeValue::String(self.tokenizer.next.token.clone()), vec![]))
        }
        else{
            match self.tokenizer.next.token.as_str(){
                "+" => {
                    self.tokenizer.select_next();
                    let factor_rec_node = self.factor();
                    let operation_node: UnOp = UnOp::new(NodeValue::Char('+'), vec![factor_rec_node]);
                    return Nodes::UnOp(operation_node)
                },
                "-" => {
                    self.tokenizer.select_next();
                    let factor_rec_node = self.factor();
                    let operation_node: UnOp = UnOp::new(NodeValue::Char('-'), vec![factor_rec_node]);
                    return Nodes::UnOp(operation_node)
                },

                "(" => {
                    self.tokenizer.select_next();
                    let inside_of_parenthesis = self.boolean_expression();
                    if self.tokenizer.next.token.as_str() != ")"{
                        println!("Erro, parentesis não fechado");
                    }
                    self.tokenizer.select_next();
                    return inside_of_parenthesis;
                },
                _ => {println!("Erro, símbolo {} inválido", &self.tokenizer.next.token)}
            }
        }
        self.tokenizer.select_next();
        return  Nodes::NoOp(NoOp::new(NodeValue::Int(0), vec![]))
    }
    fn term(&mut self) -> Nodes{
        let mut top_node = self.factor();
        loop {
            match self.tokenizer.next.token.as_str() {
                "*" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.factor();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('*'), vec![left_node, right_node]));
                }
                "/" => {
                    self.tokenizer.select_next();
                    let left_node = top_node;
                    let right_node = self.factor();
                    top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('/'), vec![left_node, right_node]));
                }
                _ => break,
            }
        }
        return top_node;
    }
    fn parse_expression(&mut self) -> Nodes{
    //println!("Token soma 1: {}", self.tokenizer.next.token.clone());
    let mut top_node = self.term();

    loop {
        //println!("Token soma 2: {}", self.tokenizer.next.token.clone());
        match self.tokenizer.next.token.as_str() {
            "+"=> {
                self.tokenizer.select_next();
                let left_node = top_node;
                let right_node = self.term();
                top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('+'), vec![left_node, right_node]));
            }
            "-" => {
                self.tokenizer.select_next();
                let left_node = top_node;
                let right_node = self.term();
                top_node = Nodes::BinOp(BinOp::new(NodeValue::Char('-'), vec![left_node, right_node]));
            }
            _ => break,
        }
    }

    return top_node; 
    }
    fn return_asf(&mut self) -> Nodes{
        self.tokenizer.select_next();
        let asf_root: Nodes;
        asf_root = self.program();
        return asf_root
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        println!("Argumento não fornecido!");
        return
    }
    let unprocessed_code = fs::read_to_string(args[1].clone()).unwrap();
    let processed_string = pre_pro_filter(unprocessed_code);
    //println!("{}",processed_string);
    let mut symbol_table : SymbolTable = SymbolTable { variables: HashMap::new() };
    let mut function_table : FuncTable = FuncTable { functions: HashMap::new() };
    let mut parser: Parser = Parser::new(processed_string, &symbol_table);
    let asf_root = parser.return_asf();
    //println!("\n inicio da avaliação \n");
    let mut playermap = Playermap::new();
    asf_root.evaluate(&mut symbol_table, &mut function_table, &mut playermap);
}

