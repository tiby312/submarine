mod display1;

use display1::*;

//use dinotreedemomenu::dinotreedemo::dinotree::axgeom;

use demodesktopgraphics::GlSys;
use demodesktopgraphics::Vertex;
//use dinotreedemomenu::*;
use dinotreedemo::dinotree::axgeom;

use ascii_num::symbol::*;
use ascii_num::*;

use axgeom::Vec2;

use axgeom::vec2;
use dinotreedemo::compute_border;


pub struct Symbols{
    digit_table:ascii_num::digit::DigitSymbolTable,
    game_table:ascii_num::GameSymbolTable,
}
impl Symbols{
    pub fn new()->Symbols{
        Symbols{
            digit_table:ascii_num::digit::DigitSymbolTable::new_default(),
            game_table:ascii_num::GameSymbolTable::new()
        }
    }
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(num_cpus::get_physical()).build_global().unwrap();
     
    let mut events_loop = glutin::EventsLoop::new();
    

    //let (mut botsys,game_response)=MenuGame::new();
    let symbols=Symbols::new();
    let (mut display1,game_response)=Display1::new(&symbols);


    let mut glsys=GlSys::new(&events_loop);
    

    let (startx,starty)=glsys.get_dim();
    let mut border=compute_border(game_response.new_game_world.unwrap().0,[startx as f32,starty as f32]);
    let radius=game_response.new_game_world.unwrap().1;
    

    glsys.set_camera_and_bot_radius(border,radius);
    glsys.set_bot_color(game_response.color.unwrap());
    

    struct Ba{
        pos:Vec2<f32>,
        id:u64
    }

    let mut running=true;
    let mut mousepos=vec2(0.0,0.0);
    let mut mouse_active=false;
    let mut poses:Vec<Ba>=Vec::new(); 
    

    loop {  
        
        if !running{
            return;
        }

        let mut keystrokes=Vec::new();

        events_loop.poll_events(|event| {
            match event {
                /*
                if let Some(Button::Keyboard(key)) = e.press_args() {
                    if key == Key::N {
                        curr=demo_iter.next(area);
                    }

                    if key == Key::C{
                        
                        check_naive= !check_naive;
                        if check_naive{
                            println!("Naive checking is on. Some demo's will now check the tree algorithm against a naive non tree version");
                        }else{
                            println!("Naive checking is off.");
                        }
                        
                    }
                };
                */
                
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput{input,..}=>{
                        if input.state==glutin::ElementState::Released{
                            if let Some(k)=input.virtual_keycode{
                                keystrokes.push(k);
                            }
                        }
                    },
                    glutin::WindowEvent::CloseRequested => {running=false},
                    glutin::WindowEvent::Resized(_logical_size) => {
                        
                    },
                    
                    glutin::WindowEvent::CursorMoved{modifiers:_,device_id:_,position:logical_position} => {
                        let glutin::dpi::LogicalPosition{x,y}=logical_position;
                        mousepos=vec2(x as f32,y as f32);
                    },

                    glutin::WindowEvent::MouseInput{modifiers:_,device_id:_,state,button}=>{
                        if button==glutin::MouseButton::Left{
                            match state{
                                glutin::ElementState::Pressed=>{  
                                    mouse_active=true;  
                                    
                                }
                                glutin::ElementState::Released=>{
                                    mouse_active=false;
                                }
                            }
                        }
                    },
                    glutin::WindowEvent::Touch(touch)=>{
                        let glutin::dpi::LogicalPosition{x,y}=touch.location;
                        //let x=(x*0.84) as f32; //TODO why needed????
                        //let y=(y*0.84) as f32; 
                        let x=x as f32;
                        let y=y as f32;

                        match touch.phase{
                            glutin::TouchPhase::Started=>{

                                let mut found=false;
                                for i in poses.iter(){
                                    if i.id == touch.id{
                                        //panic!("There is a problem!");
                                        found=true;
                                        break;
                                    }
                                }
                                if found==false{
                                    poses.push(Ba{id:touch.id,pos:vec2(x,y)});
                                }
                            },
                            glutin::TouchPhase::Ended | glutin::TouchPhase::Cancelled=>{
                                //poses.clear();
                                poses.retain(|a|a.id!=touch.id);
                            },
                            glutin::TouchPhase::Moved=>{
                                let mut ok=false;
                                for k in poses.iter_mut(){
                                    if k.id==touch.id{
                                        k.pos=vec2(x,y);
                                        ok=true;
                                        break;
                                    }
                                }
                                
                                if ok ==false{
                                    panic!("Didnt find touch");
                                }
                              
                            }
                        }


                    },
                    glutin::WindowEvent::Refresh=>{
                        //redraw=true;
                        println!("refresh");
                    },
                    _=>{}
                },
                glutin::Event::Suspended(_)=>{
                },
                _ => {},
            }    
        });

    

        let mut va:Vec<Vec2<f32>>=poses.iter().map(|a|a.pos).collect();
        if mouse_active{
            let mouseposx=mousepos.x-(startx as f32/2.0);
            let mouseposy=mousepos.y-(starty as f32/2.0);
        
            let ((x1,x2),(y1,y2))=border.get();
            let w=x2-x1;
            let h=y2-y1;

            let mouseposx=mouseposx*(w/startx as f32);
            let mouseposy=mouseposy*(h/starty as f32);
           
            va.push(vec2(mouseposx,mouseposy));
        }


        
        
        let game_response=display1.step(&va,&border,&symbols,&keystrokes);

        match game_response.new_game_world{
            Some((rect,radius))=>{
                border=compute_border(rect,[startx as f32,starty as f32]);
                glsys.set_camera_and_bot_radius(border,radius);
            },
            _=>{}
        }
        match game_response.color{
            Some(col)=>{
                glsys.set_bot_color(col);
            },
            _=>{}
        }


        if display1.get_bots().len()!=glsys.get_num_verticies(){
            glsys.re_generate_buffer(display1.get_bots().len()); 
            println!("regen!");
        }

        glsys.update(display1.get_bots(),|a|Vertex([a.pos.x,a.pos.y]));
            
        glsys.draw();
             
        
    }
    
    
}
