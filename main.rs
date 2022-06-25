//#![windows_subsystem = "windows"] //Windows only - stops console from appearing
use eframe::{self, run_native, epi::App, egui::{Image, ImageButton, TextEdit,CentralPanel, TopBottomPanel, Button, Layout, Label, RadioButton, Sense, Ui}, NativeOptions, epaint::{Vec2, Pos2, Color32, FontId, FontFamily}, emath::Align2};
use std::io;
use std::io::Write;
use std::io::Read;
//Model Buttons/Background/etc - swap each to an imagebutton -> do we want on click button change?
//Chip Images?
//Draw dice - All 6 faces only to save space

fn main() {
    let simcheck = false; //runtime change?
    if simcheck{
        sim()
    } else {
    let player1 = Player::new("Test".to_string(),310);
    let game = Gamestate::setup();
    let app = Playsurface{players: player1,game:game};
    let mut win_option = NativeOptions::default();
    win_option.min_window_size = Some(Vec2{x:768.,y:366.}); // Generalize for font size
    win_option.initial_window_size = Some(Vec2{x:768.,y:366.});
    run_native(Box::new(app), win_option);
    }
}
struct Playsurface{ //Need a player name + stack popup
    players: Player,
    game: Gamestate
}
// Images for buttons
//Draw chips bet per option on top of button location (click chips to withdraw bet) - winnings placed next to wager/into stack (option)
impl App for Playsurface {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| { //Base (buttons and background)
            let available_height = ui.available_height();// For panels
            TopBottomPanel::top("header").show(ctx,|ui| {
                let height = available_height/8.;
                let width = (ui.available_width()-6.*ui.spacing().item_spacing.x)/5.;
                ui.horizontal(|ui| {
                    let rounds_text = format!("Current Shooters Rolls: {}", self.game.rounds_since_7);
                    ui.add_sized([width,height], Label::new(rounds_text)); // Long Indent
                    ui.add_sized([width/2.,height], Button::new("0 hits").sense(Sense{ click: if self.game.rounds_since_7%4 == 0{true} else {false}, drag: false, focusable: false })); // rename hits
                    ui.add_sized([width/2.,height], Button::new("1 hit").sense(Sense{ click: if self.game.rounds_since_7%4 == 1{true} else {false}, drag: false, focusable: false })); // Light up corrosponding roll count - unclickable buttons
                    let roll_button = ui.add_sized([width,height], Button::new("ROLL!!"));
                    if roll_button.clicked(){    
                        println!("Roll!!");
                        let roll = Dice::roll();
                        render_dice(&roll);
                        game_loop(roll, &mut self.players, &mut self.game);
                        //Render 2 dice based on roll
                    }
                    ui.add_sized([width/2.,height], Button::new("2 hits").sense(Sense{ click: if self.game.rounds_since_7%4 == 2{true} else {false}, drag: false, focusable: false }));
                    ui.add_sized([width/2.,height], Button::new("3 hits").sense(Sense{ click: if self.game.rounds_since_7%4 == 3{true} else {false}, drag: false, focusable: false }));
                    let shooter_text = format!("Runs: {}", self.game.runs);
                    ui.add_sized([width,height], Label::new(shooter_text));
                })
            });
            TopBottomPanel::top("Slam/Seven").default_height(available_height/6.).show(ctx, |ui| {
                let width = (ui.available_width()-ui.spacing().item_spacing.x)/2.;
                let height = available_height/8.;
                ui.horizontal(|ui| {
                    let mut grand_but = Button::new("Grand Slam");
                    if self.game.hits >0 && self.players.bet_direction{
                        grand_but = grand_but.sense(Sense::hover());
                    } else {
                        grand_but = grand_but;
                    }
                    let grand_slam_button = ui.add_sized([width,height], grand_but);
                    let loc = grand_slam_button.rect;
                    self.game.bet_locations.grand_slam = Pos2{x:(loc.max.x-loc.min.x)*0.75+loc.min.x,y:(loc.max.y-loc.min.y)*0.5+loc.min.y};
                    if grand_slam_button.clicked(){
                        self.players.bet_grand_slam(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Grand slam");
                    }
                    chip_render(ui, self.game.bet_locations.grand_slam, &self.players.bet.grand_slam);
                    let seven_button = ui.add_sized([width,height], Button::new("Seven Out"));
                    let loc = seven_button.rect;
                    self.game.bet_locations.seven = Pos2{x:(loc.max.x-loc.min.x)*0.25+loc.min.x,y:(loc.max.y-loc.min.y)*0.5+loc.min.y};
                    if seven_button.clicked(){
                        self.players.bet_seven(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Seven Out");
                    }
                    chip_render(ui, self.game.bet_locations.seven, &self.players.bet.seven);
                })
            });
            TopBottomPanel::top("HardWays").default_height(available_height/6.).show(ctx, |ui| {
                let width = (ui.available_width()-5.*ui.spacing().item_spacing.x)/6.;
                let height = available_height/4.;
                ui.horizontal(|ui| {
                    let hard2_button = ui.add_sized([width,height], Button::new("Hard 2"));
                    let loc = hard2_button.rect;
                    self.game.bet_locations.hard2 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard2_button.clicked(){
                        self.players.bet_hard2(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Hard 2");
                    }
                    chip_render(ui, self.game.bet_locations.hard2, &self.players.bet.hard2);
                    let hard4_button = ui.add_sized([width,height], Button::new("Hard 4"));
                    let loc = hard4_button.rect;
                    self.game.bet_locations.hard4 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard4_button.clicked(){
                        self.players.bet_hard4(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Hard 4");
                    }
                    chip_render(ui, self.game.bet_locations.hard4, &self.players.bet.hard4);
                    let hard6_button = ui.add_sized([width,height], Button::new("Hard 6"));
                    let loc = hard6_button.rect;
                    self.game.bet_locations.hard6 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard6_button.clicked(){
                        self.players.bet_hard6(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Hard 6");
                    }
                    chip_render(ui, self.game.bet_locations.hard6, &self.players.bet.hard6);
                    let hard8_button = ui.add_sized([width,height], Button::new("Hard 8"));
                    let loc = hard8_button.rect;
                    self.game.bet_locations.hard8 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard8_button.clicked(){
                        self.players.bet_hard8(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Hard 8");
                    }
                    chip_render(ui, self.game.bet_locations.hard8, &self.players.bet.hard8);
                    let hard10_button = ui.add_sized([width,height], Button::new("Hard 10"));
                    let loc = hard10_button.rect;
                    self.game.bet_locations.hard10 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard10_button.clicked(){
                        self.players.bet_hard10(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet Hard 10");
                    }
                    chip_render(ui, self.game.bet_locations.hard10, &self.players.bet.hard10);
                    let hard12_button = ui.add_sized([width,height], Button::new("Hard 12"));
                    let loc = hard12_button.rect;
                    self.game.bet_locations.hard12 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if hard12_button.clicked(){ 
                        self.players.bet_hard12(self.players.bet_size, self.players.bet_direction);
                        println!("Bet Hard 12");
                    }
                    chip_render(ui, self.game.bet_locations.hard12, &self.players.bet.hard12);
                })
            });
            TopBottomPanel::top("Place Bets").default_height(available_height/6.).show(ctx, |ui| {
                let width = (ui.available_width()-5.*ui.spacing().item_spacing.x)/6.;
                let height = available_height/4.;
                ui.horizontal(|ui| {
                    let two5_button = ui.add_sized([width,height], Button::new("2/5"));
                    let loc = two5_button.rect;
                    self.game.bet_locations.two5 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if two5_button.clicked(){
                        self.players.bet_two5(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 2/5");
                    }
                    chip_render(ui, self.game.bet_locations.two5, &self.players.bet.two5);
                    let three4_button = ui.add_sized([width,height], Button::new("3/4"));
                    let loc = three4_button.rect;
                    self.game.bet_locations.three4 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if three4_button.clicked(){
                        self.players.bet_three4(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 3/4");
                    }
                    chip_render(ui, self.game.bet_locations.three4, &self.players.bet.three4);
                    let textinput = format!("6 - current bet: {}", self.players.bet.six);
                    let six_button = ui.add_sized([width,height], Button::new(textinput));
                    let loc = six_button.rect;
                    self.game.bet_locations.six = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if six_button.clicked(){
                        self.players.bet_six(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 6");
                    }
                    chip_render(ui, self.game.bet_locations.six, &self.players.bet.six);
                    let eight_button = ui.add_sized([width,height], Button::new("8"));
                    let loc = eight_button.rect;
                    self.game.bet_locations.eight = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if eight_button.clicked(){
                        self.players.bet_eight(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 8");
                    }
                    chip_render(ui, self.game.bet_locations.eight, &self.players.bet.eight);
                    let ten11_button = ui.add_sized([width,height], Button::new("10/11"));
                    let loc = ten11_button.rect;
                    self.game.bet_locations.ten11 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if ten11_button.clicked(){
                        self.players.bet_ten11(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 10/11");
                    }
                    chip_render(ui, self.game.bet_locations.ten11, &self.players.bet.ten11);
                    let nine12_button = ui.add_sized([width,height], Button::new("9/12"));
                    let loc = nine12_button.rect;
                    self.game.bet_locations.nine12 = Pos2{x:(loc.max.x-loc.min.x)*0.5+loc.min.x,y:(loc.max.y-loc.min.y)*0.75+loc.min.y};
                    if nine12_button.clicked(){
                        self.players.bet_nine12(self.players.bet_size, self.players.bet_direction);    
                        println!("Bet 9/12");
                    }
                    chip_render(ui, self.game.bet_locations.nine12, &self.players.bet.nine12);
                })
            });
            TopBottomPanel::top("Run Bet").show(ctx, |ui| {
                let available_width = ui.available_width();
                let height = available_height/8.;
                let mut run_but = Button::new("Run Bet");
                if self.game.hits%4 >0 && self.players.bet_direction{
                    run_but = run_but.sense(Sense::hover());
                } else {
                    run_but = run_but;
                }
                let runbutton = ui.add_sized([available_width,height], run_but);
                let loc = runbutton.rect;
                self.game.bet_locations.run = Pos2{x:(loc.max.x-loc.min.x)*0.66+loc.min.x,y:(loc.max.y-loc.min.y)*0.5+loc.min.y};
                chip_render(ui, self.game.bet_locations.run, &self.players.bet.run);
                if runbutton.clicked(){
                    self.players.bet_run(self.players.bet_size, self.players.bet_direction);
                    println!("Run Bet");//need to strip off the prints!
                    //self.players = self.players.bet_run(self.players.bet_size,true)
                }
            });
            TopBottomPanel::top("Stack and Bet size").show(ctx, |ui| {
                ui.horizontal(|ui|{
                    let height = available_height/8.;
                    let width = ui.available_width();
                    ui.set_height(height);
                    ui.with_layout(Layout::left_to_right(), |ui| {
                        let stack_render = format!("Stack: {}", self.players.stack);
                        
                        ui.label(stack_render); // Left adjust
                    });
                    self.game.bet_locations.stack = Pos2{x:width/4.,y:(8.*height)};
                    ui.with_layout(Layout::right_to_left(),|ui|{
                        let response = ui.add(TextEdit::singleline(&mut self.players.bet_amount).desired_width(50.));
                        ui.label("Current Bet size:"); //Right adjust - Slider/num input - (Text Box?)
                        let takedown = ui.add(RadioButton::new(self.players.bet_direction,"Bet?"));
                        if response.changed(){
                            let test = self.players.bet_amount.parse::<u32>();
                            match test {
                                Ok(ok) => self.players.bet_size = ok,
                                Err(_ok) => self.players.bet_size =0,
                            }
                            println!("bet size: {}", self.players.bet_size);
                        }
                        if response.lost_focus(){
                            self.players.bet_amount = self.players.bet_size.to_string()
                        }
                        if takedown.clicked(){
                            self.players.bet_direction = !self.players.bet_direction;
                        }
                    });
                })
            })
        });
    }
    fn name(&self) -> &str {
        "Dice Ball"
    }
}
fn render_dice(roll:&Dice){ //Next thing - Image draw/Delay/Do with primatives?
    let sizing = Vec2{x:100.,y:100.};
    let mut dice_1: Image;
    let mut dice_2: Image;
    match roll.dice1{
        //1 =>dice_1 = ImageButton::new(, sizing),
        //2 =>dice_1 = ImageButton::new(texture_id, sizing),
        //3 =>dice_1 = ImageButton::new(texture_id, sizing),
        //4 =>dice_1 = ImageButton::new(texture_id, sizing),
        //5 =>dice_1 = ImageButton::new(texture_id, sizing),
        //6 =>dice_1 = ImageButton::new(texture_id, sizing),
        _=> println!("Error Dice1"),
    }
    match roll.dice2{
        //1 =>dice_2 = Image::new(, sizing), #Image not imagebutton - not clickable
        //2 =>dice_2 = ImageButton::new(texture_id, sizing),
        //3 =>dice_2 = ImageButton::new(texture_id, sizing),
        //4 =>dice_2 = ImageButton::new(texture_id, sizing),
        //5 =>dice_2 = ImageButton::new(texture_id, sizing),
        //6 =>dice_2 = ImageButton::new(texture_id, sizing),
        _=> println!("Error Dice2"),
    }
    //Need to load into a window
    println!("Dice1: {}, Dice2: {}", roll.dice1,roll.dice2);
    println!("Total: {}", roll.total);
}
fn sim() { //TODO - Make easier/redo - Single strategy version?
    let mut shoot_total = Vec::new();
    let mut games = 100000;
    let totalgames = 100000;
    let mut strat_1_total = Vec::new();
    let mut strat_2_total = Vec::new();
    let mut strat_3_total = Vec::new();
    let mut strat_4_total = Vec::new();
    let mut strat_5_total = Vec::new();
    let mut strat_6_total = Vec::new();
    let mut strat_7_total = Vec::new();
    let mut strat_8_total = Vec::new();
    let mut strat_9_total = Vec::new();
    let mut strat_10_total = Vec::new();
    let mut strat_1_wl = 0;
    let mut strat_2_wl = 0;
    let mut strat_3_wl = 0;
    let mut strat_4_wl = 0;
    let mut strat_5_wl = 0;
    let mut strat_6_wl = 0;
    let mut strat_7_wl = 0;
    let mut strat_8_wl = 0;
    let mut strat_9_wl = 0;
    let mut strat_10_wl = 0;
    let mut strat_1_winavg = 0;
    let mut strat_1_lossavg = 0;
    let mut strat_2_winavg = 0;
    let mut strat_2_lossavg = 0;
    let mut strat_3_winavg = 0;
    let mut strat_3_lossavg = 0;
    let mut strat_4_winavg = 0;
    let mut strat_4_lossavg = 0;
    let mut strat_5_winavg = 0;
    let mut strat_5_lossavg = 0;
    let mut strat_6_winavg = 0;
    let mut strat_6_lossavg = 0;
    let mut strat_7_winavg = 0;
    let mut strat_7_lossavg = 0;
    let mut strat_8_winavg = 0;
    let mut strat_8_lossavg = 0;
    let mut strat_9_winavg = 0;
    let mut strat_9_lossavg = 0;
    let mut strat_8_maxloss = 0;
    let mut strat_9_maxloss = 0;
    let mut strat_10_winavg = 0;
    let mut strat_10_lossavg = 0;
    let mut strat_10_maxloss = 0;
    while games > 0{
        let mut live_shooter = true;
        let mut strat_1: i64 = 0;
        let mut strat_2: i64 = 0;
        let mut strat_3: i64 = 0;
        let mut strat_4: i64 = 0;
        let mut strat_5: i64 = 0;
        let mut strat_6: i64 = 0;
        let mut strat_7: i64 = 0;
        let mut strat_8: i64 = 0;
        let mut strat_9: i64 = 0;
        let mut strat_10: i64 = 0;
        let mut hands = 100;
        let mut strat_8_bet = 8;
        let mut strat_8_loss: u8 = 0;
        let mut strat_9_bet = 8;
        let mut strat_9_loss: u8 = 0;
        let mut strat_10_bet = 8;
        let mut strat_10_loss: u8 = 0;
        let mut strat_8_max = false;
        let mut strat_9_max = false;
        let mut strat_10_max = false;
        while hands > 0{
            let mut runs = 0;
            let mut rounds = 0;
            if strat_8_loss >0{
                if strat_8_loss%2 == 0{
                    strat_8_bet = strat_8_bet*2
                }
            } else {
                strat_8_bet = 8;
            }
            if strat_8 < -500{
                strat_8_max = true;
                strat_8_bet = 0
            }
            if strat_9_loss >0{
                if strat_9_loss%2 == 0{
                    strat_9_bet = strat_9_bet*2
                }
            } else {
                strat_9_bet = 8;
            }
            if strat_9 < -500{
                strat_9_max = true;
                strat_9_bet = 0
            }
            if strat_10_loss >0{
                if strat_10_loss%2 == 0{
                    strat_10_bet = strat_10_bet*2
                }
            } else {
                strat_10_bet = 8;
            }
            if strat_10 < -500{
                strat_10_max = true;
                strat_10_bet = 0
            }
            //Bet for each strategy is placed between the two whiles - modify bets from current format to simulate actual play rather than theoretical W/L
            while live_shooter{
                let out = hand(runs,Dice::roll()); 
                let winning_table = out.0;  
                runs = out.1;
                if winning_table.hard6 > 1{
                    
                    strat_7 += 15*5;
                }
                if winning_table.hard8 > 1 {
                    strat_7 += 15*5;
                }
                if winning_table.seven > 0{
                    live_shooter = false;
                    strat_7 -= 30;
                }
                rounds +=1;
                hands -=1;
            } 
            live_shooter = true;
            //Strategies
            //500 bankroll
            match rounds{
                1 => {strat_1 -= 30; strat_2 -= 30; strat_3 -= 30; strat_4 -= 30; strat_5 -=30; strat_6 -= 30; strat_8 -= strat_8_bet; strat_8_loss += 1; strat_9 -= strat_9_bet; strat_9_loss += 1; strat_10 -= strat_10_bet; strat_10_loss += 1},
                2 => {strat_1 -= 25; strat_2 -= 30; strat_3 -= 30; strat_4 -= 30; strat_5 += 5; strat_6 -= 30;strat_8 -= strat_8_bet; strat_8_loss += 1; strat_9 -= strat_9_bet; strat_9_loss += 1; strat_10 -= strat_10_bet; strat_10_loss += 1},
                3 => {strat_1 += 10; strat_2 -= 30; strat_3 -= 30; strat_4 -= 30; strat_5 += 5; strat_6 -= 30;strat_8 -= strat_8_bet; strat_8_loss += 1; strat_9 -= strat_9_bet; strat_9_loss += 1; strat_10 -= strat_10_bet; strat_10_loss += 1},
                4 => {strat_1 += 10; strat_2 -= 30; strat_3 -= 30; strat_4 -= 30; strat_5 += 5; strat_6 -= 30; strat_8 -= strat_8_bet; strat_8_loss += 1;strat_9 -= strat_9_bet; strat_9_loss += 1; strat_10 -= strat_10_bet; strat_10_loss += 1},
                5..=8 => {strat_1 += 10; strat_2 -= 15; strat_3 += 30; strat_5 += 5; strat_6 -= 30; strat_8 -= strat_8_bet; strat_8_loss += 1; strat_9 += strat_9_bet; strat_9_loss = 0; if strat_10_bet == 8 {strat_10 += strat_10_bet; strat_10_loss =0} else {strat_10 -= strat_10_bet; strat_10_loss += 1}},
                9 => {strat_1 += 10; strat_2 += 75; strat_3 += 30; strat_4 += 60; strat_5 += 5; strat_6 += 90; strat_8 += 2*strat_8_bet; strat_8_loss = 0; strat_9 += strat_9_bet; strat_9_loss = 0; if strat_10_bet == 8 {strat_10 += strat_10_bet; strat_10_loss =0} else {strat_10 += 2*strat_10_bet; strat_10_loss = 0}},
                _=> {strat_1 += 10; strat_2 += 75; strat_3 += 30; strat_4 += 60; strat_5 += 5; strat_6 += 90; strat_8 += 2*strat_8_bet; strat_8_loss = 0; strat_9 += strat_9_bet; strat_9_loss = 0; if strat_10_bet == 8 {strat_10 += strat_10_bet; strat_10_loss =0} else {strat_10 += 2*strat_10_bet; strat_10_loss = 0}},
            }
            shoot_total.push(rounds);
        }
        if strat_1 >0{
            strat_1_wl +=1;
            strat_1_winavg += strat_1;
        } else {
            strat_1_lossavg += strat_1;
        }
        if strat_2 >0{
            strat_2_wl +=1;
            strat_2_winavg += strat_2;
        } else {
            strat_2_lossavg += strat_2;
        }
        if strat_3 >0{
            strat_3_wl +=1;
            strat_3_winavg += strat_3;
        } else {
            strat_3_lossavg += strat_3;
        }
        if strat_4 >0{
            strat_4_wl +=1;
            strat_4_winavg += strat_4;
        } else {
            strat_4_lossavg += strat_4;
        }
        if strat_5 >0{
            strat_5_wl +=1;
            strat_5_winavg += strat_5;
        } else {
            strat_5_lossavg += strat_5;
        }
        if strat_6 >0{
            strat_6_wl +=1;
            strat_6_winavg += strat_6;
        } else {
            strat_6_lossavg += strat_6;
        }
        if strat_7 >0{
            strat_7_wl +=1;
            strat_7_winavg += strat_7;
        } else {
            strat_7_lossavg += strat_7;
        }
        if strat_8 >0{
            strat_8_wl +=1;
            strat_8_winavg += strat_8;
        } else {
            strat_8_lossavg += strat_8;
        }
        if strat_9 >0{
            strat_9_wl +=1;
            strat_9_winavg += strat_9;
        } else {
            strat_9_lossavg += strat_9;
        }
        if strat_8_max{
            strat_8_maxloss +=1
        }
        if strat_9_max{
            strat_9_maxloss +=1
        }
        if strat_10_max{
            strat_10_maxloss +=1
        }
        if strat_10 >0{
            strat_10_wl +=1;
            strat_10_winavg += strat_10;
        } else {
            strat_10_lossavg += strat_10;
        }
        strat_1_total.push(strat_1);
        strat_2_total.push(strat_2);
        strat_3_total.push(strat_3);
        strat_4_total.push(strat_4);
        strat_5_total.push(strat_5);
        strat_6_total.push(strat_6);
        strat_7_total.push(strat_7);
        strat_8_total.push(strat_8);
        strat_9_total.push(strat_9);
        strat_10_total.push(strat_10);
        //println!("P/L of playing board single roll: {}", strat_5);
        //println!("P/L of playing board 2 rolls: {}",strat_1);
        //println!("P/L of playing board 3 rolls: {}", strat_2);
        //println!("P/L of playing single run: {}", strat_3);
        //println!("P/L of playing double run: {}", strat_4);
        //println!("P/L of playing run single press: {}", strat_6);
        if games%1000 == 0{
            println!("{}", games);
            
        }
        games -=1;
    }
    println!("\n\n");
    println!("board single roll");
    average_and_stdev(strat_5_total);
    println!("Percentage of games profitable: {:.2}%", (strat_5_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_5_winavg as f64/strat_5_wl as f64)/500 as f64)*100 as f64, ((strat_5_lossavg as f64/(totalgames - strat_5_wl) as f64)/500 as f64)*100 as f64);
    println!("\nboard 2 rolls");
    average_and_stdev(strat_1_total);
    println!("Percentage of games profitable: {:.2}%", (strat_1_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_1_winavg as f64/strat_1_wl as f64)/500 as f64)*100 as f64, ((strat_1_lossavg as f64/(totalgames - strat_1_wl) as f64)/500 as f64)*100 as f64);
    println!("\n50% press run");
    average_and_stdev(strat_2_total);
    println!("Percentage of games profitable: {:.2}%", (strat_2_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_2_winavg as f64/strat_2_wl as f64)/500 as f64)*100 as f64, ((strat_2_lossavg as f64/(totalgames - strat_2_wl) as f64)/500 as f64)*100 as f64);
    println!("\nsingle run");
    average_and_stdev(strat_3_total);
    println!("Percentage of games profitable: {:.2}%", (strat_3_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_3_winavg as f64/strat_3_wl as f64)/500 as f64)*100 as f64, ((strat_3_lossavg as f64/(totalgames - strat_3_wl) as f64)/500 as f64)*100 as f64);
    println!("\ndouble run");
    average_and_stdev(strat_4_total);
    println!("Percentage of games profitable: {:.2}%", (strat_4_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_4_winavg as f64/strat_4_wl as f64)/500 as f64)*100 as f64, ((strat_4_lossavg as f64/(totalgames - strat_4_wl) as f64)/500 as f64)*100 as f64);
    println!("\nrun single press");
    average_and_stdev(strat_6_total);
    println!("Percentage of games profitable: {:.2}%", (strat_6_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_6_winavg as f64/strat_6_wl as f64)/500 as f64)*100 as f64, ((strat_6_lossavg as f64/(totalgames - strat_6_wl) as f64)/500 as f64)*100 as f64);
    println!("\nHard 6/Hard 8");
    average_and_stdev(strat_7_total);
    println!("Percentage of games profitable: {:.2}%", (strat_7_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_7_winavg as f64/strat_7_wl as f64)/500 as f64)*100 as f64, ((strat_7_lossavg as f64/(totalgames - strat_7_wl) as f64)/500 as f64)*100 as f64);
    println!("\nProgession - press");
    average_and_stdev(strat_8_total);
    println!("Percentage of games profitable: {:.2}%", (strat_8_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_8_winavg as f64/strat_8_wl as f64)/500 as f64)*100 as f64, ((strat_8_lossavg as f64/(totalgames - strat_8_wl) as f64)/500 as f64)*100 as f64);
    println!("Percentage of games with Max Loss: {:.2}%", (strat_8_maxloss as f64 /totalgames as f64)*100.);
    println!("\nProgession - take");
    average_and_stdev(strat_9_total);
    println!("Percentage of games profitable: {:.2}%", (strat_9_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_9_winavg as f64/strat_9_wl as f64)/500 as f64)*100 as f64, ((strat_9_lossavg as f64/(totalgames - strat_9_wl) as f64)/500 as f64)*100 as f64);
    println!("Percentage of games with Max Loss: {:.2}%", (strat_9_maxloss as f64 /totalgames as f64)*100.);
    println!("\nProgession - take on first, press on rest");
    average_and_stdev(strat_10_total);
    println!("Percentage of games profitable: {:.2}%", (strat_10_wl as f64/totalgames as f64)*100 as f64);
    println!("Avg % increase in winning games: {:.2}%, avg % loss in losing games: {:.2}%", ((strat_10_winavg as f64/strat_10_wl as f64)/500 as f64)*100 as f64, ((strat_10_lossavg as f64/(totalgames - strat_10_wl) as f64)/500 as f64)*100 as f64);
    println!("Percentage of games with Max Loss: {:.2}%", (strat_10_maxloss as f64 /totalgames as f64)*100.);
    pause()
}
fn average_and_stdev(intake: Vec<i64>){
    let mut sum: i64 = 0;
    let average:f64;
    let mut sum_difference: f64 = 0.0;
    let total = intake.len();
    for x in &intake {
        sum = sum + x;
    } 
    average = sum as f64/total as f64;
    for x in intake{
        let difference: f64;
        difference = x as f64 -average;
        sum_difference = sum_difference+(difference*difference);
    }
    let std_dev: f64 = (sum_difference/total as f64).sqrt();
    println!("The average game return for this strategy (500 bankroll per game) is {:.3} with standard deviation {:.3}", average,std_dev);
}
fn chip_render(ui: &Ui,loc:Pos2,size:&u32){
    if size < &100 && size > &0{
        ui.painter().circle(loc, 12., Color32::RED,(1.,Color32::BLACK));
        ui.painter().text(loc, Align2::CENTER_CENTER, size, FontId { size: 16., family: FontFamily::Proportional }, Color32::BLACK);
    } else if size >= &100{
        let size_hundreds = size/100;
        let size_else = size-(size_hundreds)*100;
        ui.painter().circle(Pos2 { x: loc.x-12., y: loc.y }, 12., Color32::GREEN,(2.,Color32::BLACK));
        ui.painter().text(Pos2 { x: loc.x-12., y: loc.y }, Align2::CENTER_CENTER, (size_hundreds)*100, FontId { size: 12., family: FontFamily::Proportional }, Color32::BLACK);
        if size_else > 0{ //does 1g 25 r look better than drawing 100g 25r for 125?
            ui.painter().circle(Pos2 { x: loc.x+12., y: loc.y }, 12., Color32::RED,(2.,Color32::BLACK));
            ui.painter().text(Pos2 { x: loc.x+12., y: loc.y }, Align2::CENTER_CENTER, size_else, FontId { size: 16., family: FontFamily::Proportional }, Color32::BLACK);
        }
    } 
}
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
fn game_loop(roll:Dice,players: &mut Player, game: &mut Gamestate){
    let round = game.hits;
    let is7:bool;
    if roll.total == 7{
        is7 = true;
    } else{
        is7 = false;
    }
    let wintable = hand(round, roll);
    players.bet.solve(&wintable.0);
    if is7{    
        game.hits = wintable.1;
        game.runs = wintable.1/4;
        game.rounds_since_7 = 0;
    } else{
        game.hits = wintable.1;
        game.runs = wintable.1/4;
        game.rounds_since_7 += 1;
    }
}

//Bets (take player) -> return updated player
//Print current stack and bets
//Asks to select bet types
//Once one is selected, ask bet or withdraw and size
//call corresponding betting script
//loop until done is passed
//once player is done, return updated struct
//TODO - Restrict run bet changes to between runs - Ask if they would like to bet before printing all bets
struct Gamestate{
    rounds_since_7: u32,
    hits: u8,
    runs: u8,
    bet_locations:BetSpots 
}
struct BetSpots{
    hard2:Pos2,
    hard4:Pos2,
    hard6:Pos2,
    hard8:Pos2,
    hard10:Pos2,
    hard12:Pos2,
    two5:Pos2,
    three4:Pos2,
    six:Pos2,
    eight:Pos2,
    ten11:Pos2,
    nine12:Pos2,
    run:Pos2,
    seven:Pos2,
    grand_slam:Pos2,
    stack:Pos2,
}
impl Gamestate{
    fn setup() -> Gamestate{
        Gamestate { rounds_since_7: 0, hits: 0, runs: 0, bet_locations:BetSpots{hard2:Pos2 { x: 0., y: 0. },
            hard4:Pos2 { x: 0., y: 0. },
            hard6:Pos2 { x: 0., y: 0. },
            hard8:Pos2 { x: 0., y: 0. },
            hard10:Pos2 { x: 0., y: 0. },
            hard12:Pos2 { x: 0., y: 0. },
            two5:Pos2 { x: 0., y: 0. },
            three4:Pos2 { x: 0., y: 0. },
            six:Pos2 { x: 0., y: 0. },
            eight:Pos2 { x: 0., y: 0. },
            ten11:Pos2 { x: 0., y: 0. },
            nine12:Pos2 { x: 0., y: 0. },
            run:Pos2 { x: 0., y: 0. },
            seven:Pos2 { x: 0., y: 0. },
            grand_slam:Pos2 { x: 0., y: 0. },stack:Pos2 { x: 0., y: 0. }}}
    }
}
fn hand(round:u8,roll:Dice) -> (Table,u8){
    let mut pass = round;
    // return winning table
    let mut wintable = Table{
        hard2:1,
        hard4:1,
        hard6:1,
        hard8:1,
        hard10:1,
        hard12:1,
        two5:1,
        three4:1,
        six:1,
        eight:1,
        ten11:1,
        nine12:1,
        run:1,
        seven:1,
        grand_slam:1};
    if roll.dice1 == roll.dice2{
        match roll.total{
            2 => {pass +=1; wintable = Table{
                hard2:6,two5:2,seven:0,..wintable}},//Hard 2 + 2/5 wins
            4 => {pass +=1; wintable = Table{
                    hard4:6,three4:2,seven:0,..wintable}},//Hard 4 +3/4 wins
            6 => {pass +=1; wintable = Table{ 
                hard6:6, six:2, seven:0,..wintable}},//Hard 6 + soft 6 wins +1 pass
            8 => {pass +=1; wintable = Table{ 
                hard8:6, eight:2, seven:0,..wintable}},//Hard 8 + soft 8 wins +1 pass
            10 => {pass +=1; wintable = Table{ 
                hard10:6, ten11:2, seven:0,..wintable}},//hard 10 +10/11 wins +1 pass
            12 => {pass +=1; wintable = Table{ 
                hard12:6, nine12:2, seven:0,..wintable}},//hard 12 + 9/12 wins +1 pass
            _ => println!("Hard error"),// No win (should never occur)
        }
    } else {
        match roll.total {
            2 => {pass +=1; wintable = Table{ 
                two5:2, seven:0,..wintable}},// 2/5 wins +1 pass
            3 => {pass +=1; wintable = Table{ 
                three4:2, seven:0,..wintable}},// 3/4 wins +1 pass
            4 => {pass +=1; wintable = Table{ 
                three4:2, seven:0,..wintable}},// 3/4 wins +1 pass
            5 => {pass +=1; wintable = Table{ 
                two5:2, seven:0,..wintable}},// 2/5 wins +1 pass
            6 => {pass +=1; wintable = Table{ 
                six:2, seven:0,..wintable}},// soft 6 wins +1 pass
            7 => {pass = 0; wintable = Table{ 
                    hard2:0,
                    hard4:0,
                    hard6:0,
                    hard8:0,
                    hard10:0,
                    hard12:0,
                    two5:0,
                    three4:0,
                    six:0,
                    eight:0,
                    ten11:0,
                    nine12:0,
                    run:0,
                    seven:5,
                    grand_slam:0}},//7 up wins + pass goes to 0 + Payout Streak + all other bets lost
            8 => {pass +=1; wintable = Table{ 
                eight:2, seven:0, ..wintable}},// soft 8 wins +1 pass
            9 => {pass +=1; wintable = Table{ 
                nine12:2, seven:0, ..wintable}},// 9/12 wins +1 pass
            10 => {pass +=1; wintable = Table{ 
                ten11:2, seven:0, ..wintable}},// 10/11 wins +1 pass
            11 => {pass +=1; wintable = Table{ 
                ten11:2, seven:0, ..wintable}},// 10/11 wins +1 pass
            12 => {pass +=1; wintable = Table{ 
                nine12:2, seven:0, ..wintable}},// 9/12 wins +1 pass
            _=> println!("soft error"),// No win (should never occur)
        }
    }
    if pass%4 == 0 && pass != 0{
        wintable = Table{ 
            run:2, ..wintable};
        match pass/4{
            1..=3 => (),
            4 => {wintable = Table{ 
                grand_slam:5, ..wintable}},
            5 => {wintable = Table{ 
                grand_slam:15, ..wintable}},
            6 => {pass = 0; wintable = Table{ 
                grand_slam:35, ..wintable}}, //pass resets to 0
            _ => println!("Grand Slam error"), //should never occur
        }
    }
    (wintable,pass)
}
struct Table{
    hard2:u32,
    hard4:u32,
    hard6:u32,
    hard8:u32,
    hard10:u32,
    hard12:u32,
    two5:u32,
    three4:u32,
    six:u32,
    eight:u32,
    ten11:u32,
    nine12:u32,
    run:u32,
    seven:u32,
    grand_slam:u32,
}
impl Table{
    fn solve(&mut self,winner:&Table){
            self.hard2=winner.hard2*self.hard2;
            self.hard4=winner.hard4*self.hard4;
            self.hard6=winner.hard6*self.hard6;
            self.hard8=winner.hard8*self.hard8;
            self.hard10=winner.hard10*self.hard10;
            self.hard12=winner.hard12*self.hard12;
            self.two5=winner.two5*self.two5;
            self.three4=winner.three4*self.three4;
            self.six=winner.six*self.six;
            self.eight=winner.eight*self.eight;
            self.ten11=winner.ten11*self.ten11;
            self.nine12=winner.nine12*self.nine12;
            self.run=winner.run*self.run;
            self.seven=winner.seven*self.seven;
            self.grand_slam=winner.grand_slam*self.grand_slam;
    }
}
struct Player{
    stack: u32,
    bet_size: u32,
    bet: Table,
    _name: String,
    bet_amount: String,
    bet_direction: bool,
}
impl Player{
    fn new(name:String,start_stack:u32) -> Player{
        Player{stack:start_stack,bet_size: 0, bet:Table{
            hard2:0,
            hard4:0,
            hard6:0,
            hard8:0,
            hard10:0,
            hard12:0,
            two5:0,
            three4:0,
            six:0,
            eight:0,
            ten11:0,
            nine12:0,
            run:0,
            seven:0,
            grand_slam:0
        },_name:name,bet_amount: "0".to_string(),bet_direction: true}
    }
    fn bet_run(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.run += size;
            }
        }else {
            if size <= self.bet.run{
                self.bet.run -= size;
                self.stack += size;
            }
        }
    }
    fn bet_grand_slam(& mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.grand_slam += size;
            }
        }else {
            if size <= self.bet.grand_slam{
                self.stack +=size;
                self.bet.grand_slam -= size;
            }
        }
    }
    fn bet_seven(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.seven += size;
            }
        }else {
            if size <= self.bet.seven{
               self.stack += size;
               self.bet.seven -= size;
            }
        }
    }
    fn bet_hard2(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.hard2 += size;
            }
        }else {
            if size <= self.bet.hard2{
                self.stack+=size;
                self.bet.hard2 -= size;
            }
        }
    }
    fn bet_hard4(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
               self.stack -= size;
               self.bet.hard4 += size;
            }
        }else {
            if size <= self.bet.hard4{
                self.stack += size;
                self.bet.hard4 -= size;
            }
        }
    }
    fn bet_hard6(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.hard6 += size;
            }
        }else {
            if size <= self.bet.hard6{
                self.stack += size;
                self.bet.hard6 -= size;
            }
        }
    }
    fn bet_hard8(&mut self,size: u32,direction:bool){
        if direction{
            self.bet.hard8 += size;
            self.stack -= size; 
        } else {
            self.bet.hard8 -= size;
            self.stack += size; 
        }
    }
    fn bet_hard10(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.hard10 += size;
            }
        }else {
            if size <= self.bet.hard10{
                self.stack += size;
                self.bet.hard10 -= size;
            }
        }
    }
    fn bet_hard12(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.hard12 += size;
            }
        }else {
            if size <= self.bet.hard12{
                self.stack += size;
                self.bet.hard12 -= size;
            }
        }
    }
    fn bet_six(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.six += size;
            }
        }else {
            if size <= self.bet.six{
                self.stack += size;
                self.bet.six -= size;
            }
        }
    }
    fn bet_eight(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.eight += size;
            }
        }else {
            if size <= self.bet.eight{
                self.stack += size;
                self.bet.eight -= size;
            }
        }
    }
    fn bet_two5(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.two5 += size;
            }
        }else {
            if size <= self.bet.two5{
                self.stack += size;
                self.bet.two5 -= size;
            }
        }
    }
    fn bet_three4(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.three4 += size;
            }
        }else {
            if size <= self.bet.three4{
                self.stack += size;
                self.bet.three4 -= size;
            }
        }
    }
    fn bet_ten11(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.ten11 += size;
            }
        }else {
            if size <= self.bet.ten11{
                self.stack += size;
                self.bet.ten11 -= size;
            }
        }
    }
    fn bet_nine12(&mut self,size: u32,direction:bool){
        if direction{
            if size <= self.stack{
                self.stack -= size;
                self.bet.nine12 += size;
            }
        }else {
            if size <= self.bet.nine12{
                self.stack += size;
                self.bet.nine12 -= size;
            }
        }
    }
}


struct Dice{
    dice1: u8,
    dice2: u8,
    total: u8,
}
impl Dice{
    fn roll() -> Dice{
        let dice_faces  = vec![1,2,3,4,5,6];
        let first_roll = fastrand::usize(..dice_faces.len());
        let second_roll = fastrand::usize(..dice_faces.len());
        let first_dice = dice_faces[first_roll];
        let second_dice = dice_faces[second_roll];
        let amount = first_dice+second_dice;
        Dice{dice1:first_dice,dice2:second_dice,total:amount}
    }
}