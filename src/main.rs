mod display1;
use display1::*;

pub use demodesktopgraphics::glutin;

use demodesktopgraphics::GlSys;
use demodesktopgraphics::Vertex;
use dinotreedemo::dinotree::axgeom;



use axgeom::Vec2;
use axgeom::vec2;
use dinotreedemo::compute_border;

use glutin::event::WindowEvent;
use glutin::event::ElementState;
use glutin::event::VirtualKeyCode;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;



pub static COLOR_TABLE:&'static [[f32;3]]=
    &[
        [0.0,1.0,0.0],
        [0.9,0.9,0.9],
        [1.0,0.2,0.2],
        [0.6,0.5,1.0],
        [1.0,1.0,0.0],
        [1.0,0.0,1.0],
        [0.0,1.0,1.0],
    ];

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
     
    let events_loop = glutin::event_loop::EventLoop::new();
    
    let symbols=Symbols::new();
    let (display1,game_response)=Display1::new(&symbols);

    let mut display1:Box<dyn MenuTrait>=Box::new(display1);
    let mut glsys=GlSys::new(&events_loop);
    

    let (startx,starty)=glsys.get_dim();
    let mut border=compute_border(game_response.new_game_world.unwrap().0,[startx as f32,starty as f32]);
    let radius=game_response.new_game_world.unwrap().1;
    

    glsys.set_camera_and_bot_radius(border,radius);
    
    
    let mut bot_buffer=glsys.create_vbo(0);
    

    let mut mousepos=vec2(0.0,0.0);
    #[derive(Clone,Debug)]
    struct GameInputs{
        keystrokes:Vec<VirtualKeyCode>,
        mouseposes:Vec<Vec2<f32>>
    }

    let mut color_table_counter=0;

    let mut game_inputs=GameInputs{keystrokes:Vec::new(),mouseposes:Vec::new()};
    let mut mouse_pressed=false;

    let mut color_alpha=0.0;
    //TODO talk to glutin about why there is a static lifetime bound.
    let mut last_time:Option<std::time::Instant>=None;
    
    events_loop.run(move |event,_,control_flow| {
        match event {

            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput{input,..}=>{
                            
                    match input.virtual_keycode{
                        Some(VirtualKeyCode::Escape)=>{
                            *control_flow=ControlFlow::Exit;
                        },
                        _=>{}
                    }

                    if input.state==ElementState::Released{
                        if let Some(k)=input.virtual_keycode{
                            game_inputs.keystrokes.push(k);
                        }

                        if let Some(k)=input.virtual_keycode{
                            if k==VirtualKeyCode::C{
                                game_inputs.mouseposes.push(mousepos);
                                //mouse_active=true
                            }
                        }
                    }


                },
                WindowEvent::CloseRequested => {println!("close requested!");*control_flow=ControlFlow::Exit;},
                WindowEvent::Resized(_logical_size) => {
                    //glsys.set_camera_and_bot_radius(border,radius);
                },
                WindowEvent::MouseInput{device_id:_,state:ElementState::Released,button:_,modifiers:_}=>{
                    mouse_pressed=false;
                },
                WindowEvent::MouseInput{device_id:_,state:ElementState::Pressed,button:_,modifiers:_}=>{
                    mouse_pressed=true;
                    //color_table_counter=(color_table_counter+1) % COLOR_TABLE.len();
                },
                WindowEvent::CursorMoved{modifiers:_,device_id:_,position:logical_position} => {
                    let glutin::dpi::LogicalPosition{x,y}=logical_position;
                    let mousepos2=vec2(x as f32,y as f32);
                    //let mut va:Vec<Vec2<f32>>=poses.iter().map(|a|a.pos).collect();
                    
                    let mouseposx=mousepos2.x-(startx as f32/2.0);
                    let mouseposy=mousepos2.y-(starty as f32/2.0);
                
                    let ((x1,x2),(y1,y2))=border.get();
                    let w=x2-x1;
                    let h=y2-y1;

                    let mouseposx=mouseposx*(w/startx as f32);
                    let mouseposy=mouseposy*(h/starty as f32);
                   
                    mousepos=vec2(mouseposx,mouseposy);
                },
                WindowEvent::RedrawRequested => {
                    //glsys.set_camera_and_bot_radius(border,radius);
                       
                    /*
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                    */
                }
                _=>{}
            },
            Event::EventsCleared=>{
                let do_run = match last_time{
                    Some(last_time)=>{
                        if last_time.elapsed().as_millis()>=16{
                            true
                        }else{
                            false
                        }
                    },
                    None=>{
                        true
                    }
                };

                if do_run{
                    let game_response=display1.step(&game_inputs.mouseposes,&border,&symbols,&game_inputs.keystrokes);

                    
                    let game_response = if game_response.next_world{
                        let k=Display2::new(&symbols);
                        display1=Box::new(k.0);
                        k.1
                    }else{
                        game_response
                    };

                    if let Some(new_game_world)= game_response.new_game_world{
                        border=compute_border(new_game_world.0,[startx as f32,starty as f32]);
                        for _ in 0..100{
                            display1.step(&game_inputs.mouseposes,&border,&symbols,&game_inputs.keystrokes);
                        }
                    }

                    
                    if let Some(new_game_world)=game_response.new_game_world{
                        let (rect,radius)=new_game_world;
                        border=compute_border(rect,[startx as f32,starty as f32]);                            
                        glsys.set_camera_and_bot_radius(border,radius);
                    }



                    if display1.get_bots().len()!=bot_buffer.get_num_verticies(){
                        glsys.re_generate_buffer(&mut bot_buffer,display1.get_bots().len()); 
                    }


                    bot_buffer.update(&display1.get_bots(),|b|{
                        Vertex([b.pos.x,b.pos.y,color_alpha])
                    });

                    if mouse_pressed{
                        color_alpha=(color_alpha+0.02f32).min(1.0);
                    }else{
                        color_alpha=(color_alpha-0.02).max(0.0);    
                    }
                    
                    
                    let mut ss=glsys.new_draw_session([0.0,0.0,0.0]);
                    let color=COLOR_TABLE[color_table_counter];
                    ss.draw_vbo_section(&bot_buffer,0,display1.get_bots().len(),color);
                    //ss.draw_vbo_section(&bot_buffer,0,200,[1.0,0.0,3.0]);
                    //ss.draw_vbo_section(&bot_buffer,200,display1.get_bots().len(),color);
                    ss.finish();
        
                    game_inputs.mouseposes.clear();
                    game_inputs.keystrokes.clear();
                    
                    last_time=Some(std::time::Instant::now());
                
                }         
            }
            _ => {},
        }  
    });
}
