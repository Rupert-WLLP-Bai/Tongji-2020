/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int main()
{
    int i;
    int temp1, temp2, temp3, temp4;
    int i1, i2, i3, i4, i5;
    printf("请输入一个[1..30000]间的整数：\n");
    scanf("%d", &i);

    i1 = i % 10;
    temp1 = i / 10;
    i2 = temp1 % 10;
    temp2 = temp1 / 10;
    i3 = temp2 % 10;
    temp3 = temp2 / 10;
    i4 = temp3 % 10;
    temp4 = temp3 / 10;
    i5 = temp4 % 10;

    printf("万位 : %d\n", i5);
    printf("千位 : %d\n", i4);
    printf("百位 : %d\n", i3);
    printf("十位 : %d\n", i2);
    printf("个位 : %d\n", i1);

    return 0;
}

//Done.
