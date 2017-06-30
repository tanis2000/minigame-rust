//
//  ViewController.m
//  minigame
//
//  Created by Valerio Santinelli on 06/06/17.
//  Copyright © 2017 Valerio Santinelli. All rights reserved.
//

#import "ViewController.h"

@interface ViewController ()

@end

@implementation ViewController

extern void SDL_main();

- (void)viewDidLoad {
  [super viewDidLoad];
  SDL_main();
}


- (void)didReceiveMemoryWarning {
  [super didReceiveMemoryWarning];
  // Dispose of any resources that can be recreated.
}


@end
