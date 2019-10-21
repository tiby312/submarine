use dinotreedemo::dinotree::axgeom;
use crate::glutin::event::VirtualKeyCode;
use laid_dot::*;
use ascii_num::*;
use dinotreedemo::BotSystem;
use axgeom::*;
use crate::Symbols;
use dinotreedemo::duckduckgeo;
use duckduckgeo::bot::Bot;

#[derive(Debug,Clone,Copy)]
pub struct GameResponse
{
    pub color:          Option<[f32;3]>,
    pub new_game_world: Option<(Rect<f32>,f32)>,
    pub next_world : bool
}

pub struct Timer<T>{
    counter:usize,
    inner:Option<T>
}
impl<T> Timer<T>{
    pub fn new(counter:usize,inner:T)->Timer<T>{
        Timer{counter,inner:Some(inner)}
    }
    pub fn step(&mut self)->Option<T>{
        if self.inner.is_some() && self.counter==0{
            return self.inner.take();
        }
        if self.counter>0{
            self.counter-=1;
        }
        None
    }
}


pub trait MenuTrait:Send+Sync{
    fn step(&mut self,poses:&[Vec2<f32>],border:&Rect<f32>,symbols:&Symbols,keystrokes:&[VirtualKeyCode])->GameResponse;
    fn get_bots(&self)->&[Bot];
}



pub struct Display2{
    bots:BotSystem,
    numberthings:[NumberThing;4],
}

impl Display2{
    pub fn new(_symbols:&Symbols)->(Display2,GameResponse){
        let (sys,rect,radius)=BotSystem::new(40_000);
        let numberthings=[
            NumberThing::new(8,300.0,30.0,vec2(-1500.0,-300.0)),
            NumberThing::new(8,300.0,30.0,vec2(-300.0,-100.0)),
            NumberThing::new(0,300.0,30.0,vec2(100.0,300.0)),
            NumberThing::new(2,300.0,30.0,vec2(1200.0,600.0)),
        ];
        (Display2{bots:sys,numberthings},GameResponse{color:Some([0.0,1.0,0.0]),new_game_world:Some((rect,radius)),next_world:false})
    }
}

impl MenuTrait for Display2{
    fn step(&mut self,poses:&[Vec2<f32>],_border:&Rect<f32>,symbols:&Symbols,_keystrokes:&[VirtualKeyCode])->GameResponse{
        self.bots.step(poses,_border);
        let mut bb=self.bots.get_bots_mut().iter_mut();
        for numberthing in self.numberthings.iter(){
            for digit in numberthing.iter(&symbols.digit_table){
                for pos in digit{
                    let k=bb.next().unwrap();
                    k.pos=pos;
                    k.vel=vec2same(0.);
                    k.acc=vec2same(0.);
                }
            }
        }

        GameResponse{color:None,new_game_world:None,next_world:false}
    }

    fn get_bots(&self)->&[Bot]{
        self.bots.get_bots()
    }
}




pub struct Display1{
    bots: Vec<Bot>,
    buttons:[Button;3],
    color_button:Button,
    color_clicker:Clicker,
    numberthing:NumberThing,
    pin_code:PinCode,
    pin_code_counter:Option<Timer<PinEnterResult>>
}


impl MenuTrait for Display1{

    fn step(&mut self,poses:&[Vec2<f32>],_border:&Rect<f32>,symbols:&Symbols,keystrokes:&[VirtualKeyCode])->GameResponse{
        
        for k in keystrokes.iter(){
            let key = match k{
                VirtualKeyCode::Key1=>Some(1),
                VirtualKeyCode::Key2=>Some(2),
                VirtualKeyCode::Key3=>Some(3),
                VirtualKeyCode::Key4=>Some(4),
                VirtualKeyCode::Key5=>Some(5),
                VirtualKeyCode::Key6=>Some(6),
                VirtualKeyCode::Key7=>Some(7),
                VirtualKeyCode::Key8=>Some(8),
                VirtualKeyCode::Key9=>Some(9),
                VirtualKeyCode::Key0=>Some(0),
                _=>{
                    None
                }
            };

            if self.pin_code_counter.is_none(){
                if let Some(key)=key{
                    println!("processing={:?}",key);

                    match self.pin_code.add(key){
                        PinEnterResult::Fail=>{
                            //self.pin_code=NumberThingOrFlasher::NumberFlasher::new(pincode)
                            
                            //std::thread::sleep(std::time::Duration::from_secs(5));
                            self.pin_code_counter=Some(Timer::new(40,PinEnterResult::Fail));
                            //self.pin_code_counter=(true,100);
                            //self.pin_code.reset();
                        },
                        PinEnterResult::NotDoneYet=>{
                            self.pin_code_counter=None;
                        },
                        PinEnterResult::Open=>{
                            self.pin_code_counter=Some(Timer::new(1,PinEnterResult::Open));
                            
                            //self.pin_code_counter=(false,100);
                            
                        }
                    }

                }
            }



        }


        let bots=&mut self.bots;
        
        for i in poses.iter(){
            let curr=self.numberthing.get_number();

            //up arrow
            if self.buttons[0].get_dim().contains_point(*i){
                self.numberthing.update_number(curr+50);
            }
            if self.buttons[1].get_dim().contains_point(*i){
                self.numberthing.update_number((curr as isize-50).max(1) as usize); 
            }
            if self.buttons[2].get_dim().contains_point(*i){

                //let (game,rect,radius)=BotSystem::new(curr);
                //return (Some(Box::new(Game{game})),GameResponse{color:None,is_game:true,new_game_world:Some((rect,radius))})
            }
        }

        if self.color_clicker.update(self.color_button.get_dim(),poses){
            //self.col_counter=(self.col_counter+1) % COLS.len();
        }

        {
            let mut bb=bots.iter_mut();

            /*
            
            for digit in self.numberthing.iter(&symbols.digit_table){
                for pos in digit{
                    bb.next().unwrap().pos=pos;
                }
            }

        
            for i in self.buttons.iter(){
                for pos in i.iter(&symbols.game_table.0){

                    bb.next().unwrap().pos=pos;
                }
            }


            for pos in self.color_button.iter(&symbols.game_table.0){
                bb.next().unwrap().pos=pos;
            };
            */
            for d in self.pin_code.iter(&symbols.digit_table){
                for pos in d{
                    bb.next().unwrap().pos=pos;
                }
            }
            
            for b in bb{
                b.pos=vec2(-10000.0,-10000.0);
            }


        }


        if let Some(timer)=&mut self.pin_code_counter{
            if let Some(res) = timer.step(){
                match res{
                    PinEnterResult::Fail=>{
                        println!("hi alaina. that combination was a fail"); 
                        self.pin_code.reset();
                        self.pin_code_counter=None;
                    },
                    PinEnterResult::Open=>{
                        println!("hi alaina. you opened the lock");
                        return GameResponse{new_game_world:None,color:None,next_world:true}
                        //return true;
                    },
                    PinEnterResult::NotDoneYet=>{
                        unreachable!()
                    }
                }
            }
        }

        GameResponse{new_game_world:None,color:None,next_world:false}
    }

    fn get_bots(&self)->&[Bot]{
        &self.bots
    }   
}
impl Display1{

    pub fn new(symbols:&Symbols)->(Display1,GameResponse){
        
        let num_bots=5_000;
        
        let startx=500.0;
        let starty=500.0;

        //let border= axgeom::Rect::new(NotNaN::new(-startx).unwrap(),NotNaN::new(startx).unwrap(),NotNaN::new(-starty).unwrap(),NotNaN::new(starty).unwrap());
        let borderf32= axgeom::Rect::new(-startx,startx ,-starty,starty);

        //used as the building block for all positions
        let unit=8.0;//bot::get_unit(startx,starty);
        
        //let br=unit*1.0;
        //let mr=unit*10.0;

        //let (bot_prop,mouse_prop)=bot::create_from_radius(br,mr);
        //let bots=bot::create_bots(num_bots,&border,&bot_prop);
        //let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);
        //let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(&Vec2::new(pos[0] as f32,pos[1] as f32))).collect();
        let z=vec2(0.0,0.0);
        let bots:Vec<Bot>=(0..num_bots).map(|_|Bot{pos:z,vel:vec2(50.0,0.0),acc:z}).collect();



        let kk=vec2(-200.0,-100.0);
        let color_button=Button::new(kk,3,unit*2.0,&symbols.game_table.0);


        let buttons={
            let mut v=vec2(-200.0,100.0);
            let b1=Button::new(v,0,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            let b2=Button::new(v,1,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            let b3=Button::new(v,2,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            [b1,b2,b3]
        };

        /*
        let kk=Vec2::new(unit*5.0,starty as f32-unit*70.0);    
        let debug_button=OnOffButton::new(kk,
                ascii_num::get_misc(4),
                ascii_num::get_misc(5),
                unit*2.0);
        */

        let numberthing={
            let x=startx as f32-100.0;
            let y=starty as f32-200.0;
            NumberThing::new(40_000,unit*15.0,unit*2.0,vec2(x,y))
        };

        let col=[0.0,1.0,0.0];

        let pin_code=PinCode::new(vec2(50.0,50.0),100.0,10.0);

        (Display1{
            bots,
            buttons,
            color_button,
            color_clicker:Clicker::new(),
            numberthing,
            pin_code,
            pin_code_counter:None
        },GameResponse{color:Some(col),new_game_world:Some((borderf32,10.0)),next_world:false})
    }


}
