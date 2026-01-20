/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <math.h>
#define pi 3.14159
int main()
{
    int a, b, angle;
    double S;
    printf("请输入三角形的两边及其夹角（角度） :  \n");
    scanf("%d%d%d", &a, &b, &angle);
    S = a * b * sin(angle * pi / 180) / 2;
    printf("三角形面积为 : %.3f", S);
    return 0;
}

//Done.
