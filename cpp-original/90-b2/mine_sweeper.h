/*2052526 信15 白俊豪*/

#pragma once

/*雷的状态*/
#define mark -1/*已被标记*/
#define no 0/*无雷*//*未标记*/
#define yes 1/*有雷*/



/*雷阵的长宽,雷数*/
#define W_max 18/*拓宽,便于计算雷数*/
#define L_max 32
#define W1 9/*难度1*/
#define L1 9

#define W2 16/*难度2*/
#define L2 16

#define W3 16/*难度3*/
#define L3 30

#define mine1 10
#define mine2 40
#define mine3 99


/*雷阵是否被打开*/
#define OPEN 1
#define HIDE 0



/*输入长度,宽度,雷数生成对应的雷阵*/	/*传入的数组是表示有雷或无雷的数组*/
void spawn(int width, int length, int num, int (*a)[L_max],int x,int y);
/*计算雷数*/
void Num_of_mine(int(* mine)[L_max], int(* a)[L_max], int select);
/*输出雷阵*/
void print_result(int(*mine)[L_max], int(*a)[L_max], int select);


/*获取输入*/
void get_choice(int* select);
/*打印主菜单*/
void print_menu();
/*打印难度选择界面*/
void print_choose_difficulty();
/*按回车继续*/
void wait_for_enter();
/*根据输入的值选择对应的功能*/
void menu(int select, int(*mine)[L_max], int(*a)[L_max],int(*b)[L_max],int(*c)[L_max]);
/*获取难度*/
void get_difficulty(int* d);
/*情况1*/
void case1(int(*mine)[L_max], int(*a)[L_max]);
/*情况2*/
void case2(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max]);
/*情况3*/
void case3(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max]);
/*情况4*/
void case4(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max]);
/*情况5*/
void case5(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max]);
/*情况6*/
/*检查鼠标位置的合法性,输出对应的行列位置,在合法位置按左键退出*/
/*用case5生成雷阵*/
/*获取鼠标的位置与事件*/
/*输出相应的位置,左键break;*/
void case6(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max]);
/*情况7*/
void case7(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max]);
/*情况8*/
void case8(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max]);
/*情况9*/
void case9(int(*mine)[L_max], int(*a)[L_max], int(*b)[L_max], int(*c)[L_max]);
/*递归处理打开的位置*//*情况1*/
void open_matrix1(int(*a)[L_max], int(*b)[L_max], int x, int y);
/*递归处理打开的位置*//*情况2*/
void open_matrix2(int(*a)[L_max], int(*b)[L_max], int x, int y);
/*递归处理打开的位置*//*情况3*/
void open_matrix3(int(*a)[L_max], int(*b)[L_max], int x, int y);

/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况1*/
bool stop1(int(*a)[L_max],int (*b)[L_max], int x, int y);
/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况2*/
bool stop2(int(*a)[L_max], int(*b)[L_max], int x, int y);
/*周围有雷返回0,遇到边界返回0,否则返回1*//*情况3*/
bool stop3(int(*a)[L_max], int(*b)[L_max], int x, int y);

/*获取命令,对应相应的数组下标*//*分三种情况分别设定范围*/
/*MARK表示是否开启!,&,#对应的功能*//*传入两个坐标值*/
/*返回值: 0退出游戏,1打开雷阵,2标记雷阵,-1取消标记,-2显示时间*/
int fetch_instruction(int* x, int* y, int select, bool MARK);


/*雷阵重置*/
void reset(int(*mine)[L_max], int(*a)[L_max]);

/*输出初始的格子*/
void print_graph(int select);

/*整合打开雷阵的函数*/
void open_matrix(int(*a)[L_max], int(*b)[L_max], int x, int y, int select);

int game_over(int(*b)[L_max], int select);

int judge4(const int x, const int y, int(*a)[L_max], int(*b)[L_max], int(*c)[L_max]);