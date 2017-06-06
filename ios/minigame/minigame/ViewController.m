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

extern void run_loop();

- (void)viewDidLoad {
  [super viewDidLoad];
  run_loop();
}


- (void)didReceiveMemoryWarning {
  [super didReceiveMemoryWarning];
  // Dispose of any resources that can be recreated.
}


@end
