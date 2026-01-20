/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <math.h>
#include <stdbool.h>
int main()
{
    int i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i_1, i_2, temp1, temp2, temp3, temp4, temp5, temp6, temp7, temp8, temp9;
    double temp_1, num, num_int, num_decimal_part;
    bool i10_19_exist = 1, zero_1_exist = 0, zero_2_exist = 0, zheng_exist = 0;
    printf("请输入[0-100亿)之间的数字,小数点后最多两位：\n");
    scanf("%lf", &num);
    num_int = num - fmod(num, 1);
    i1 = (int)(fmod(num_int, 10));
    temp1 = (int)(num_int / 10);
    i2 = temp1 % 10;
    temp2 = temp1 / 10;
    i3 = temp2 % 10;
    temp3 = temp2 / 10;
    i4 = temp3 % 10;
    temp4 = temp3 / 10;
    i5 = temp4 % 10;
    temp5 = temp4 / 10;
    i6 = temp5 % 10;
    temp6 = temp5 / 10;
    i7 = temp6 % 10;
    temp7 = temp6 / 10;
    i8 = temp7 % 10;
    temp8 = temp7 / 10;
    i9 = temp8 % 10;
    temp9 = temp8 / 10;
    i10 = temp9 % 10;
    num_decimal_part = fmod(num, 1);
    i_1 = (int)((num_decimal_part * 10) + 1e-4);
    temp_1 = num_decimal_part * 100 + 1e-3;
    i_2 = (int)(fmod(temp_1, 10) + 1e-4);
    switch (i10) //十亿位
    {
        case 1:
            printf("壹拾");
            break;
        case 2:
            printf("贰拾");
            break;
        case 3:
            printf("叁拾");
            break;
        case 4:
            printf("肆拾");
            break;
        case 5:
            printf("伍拾");
            break;
        case 6:
            printf("陆拾");
            break;
        case 7:
            printf("柒拾");
            break;
        case 8:
            printf("捌拾");
            break;
        case 9:
            printf("玖拾");
            break;
    }
    switch (i9) //亿位
    {
        case 1:
            printf("壹");
            break;
        case 2:
            printf("贰");
            break;
        case 3:
            printf("叁");
            break;
        case 4:
            printf("肆");
            break;
        case 5:
            printf("伍");
            break;
        case 6:
            printf("陆");
            break;
        case 7:
            printf("柒");
            break;
        case 8:
            printf("捌");
            break;
        case 9:
            printf("玖");
            break;
    }
    if (i10 != 0 || i9 != 0)
        printf("亿");
    else
        i10_19_exist = 0;
    switch (i8) //千万位
    {
        case 1:
            printf("壹仟");
            break;
        case 2:
            printf("贰仟");
            break;
        case 3:
            printf("叁仟");
            break;
        case 4:
            printf("肆仟");
            break;
        case 5:
            printf("伍仟");
            break;
        case 6:
            printf("陆仟");
            break;
        case 7:
            printf("柒仟");
            break;
        case 8:
            printf("捌拾");
            break;
        case 9:
            printf("玖拾");
            break;
    }
    if (!(i8 || i7 || i6 || i5))
        zero_1_exist = 1;
    if (!zero_1_exist && i10_19_exist && !i8)
    {
        printf("零");
        zero_1_exist = 1;
    }
    switch (i7) //百万位
    {
        case 1:
            printf("壹佰");
            break;
        case 2:
            printf("贰佰");
            break;
        case 3:
            printf("叁佰");
            break;
        case 4:
            printf("肆佰");
            break;
        case 5:
            printf("伍佰");
            break;
        case 6:
            printf("陆佰");
            break;
        case 7:
            printf("柒佰");
            break;
        case 8:
            printf("捌佰");
            break;
        case 9:
            printf("玖佰");
            break;
    }
    if (!zero_1_exist && !i7 && i6 && num >= 1000000)
    {
        printf("零");
        zero_1_exist = 1;
    }
    switch (i6) //十万位
    {
        case 1:
            printf("壹拾");
            break;
        case 2:
            printf("贰拾");
            break;
        case 3:
            printf("叁拾");
            break;
        case 4:
            printf("肆拾");
            break;
        case 5:
            printf("伍拾");
            break;
        case 6:
            printf("陆拾");
            break;
        case 7:
            printf("柒拾");
            break;
        case 8:
            printf("捌拾");
            break;
        case 9:
            printf("玖拾");
            break;
    }
    if (i7 && !i6 && i5 && num >= 100000)
    {
        printf("零");
        zero_1_exist = 1;
    }
    switch (i5) //万位
    {
        case 1:
            printf("壹");
            break;
        case 2:
            printf("贰");
            break;
        case 3:
            printf("叁");
            break;
        case 4:
            printf("肆");
            break;
        case 5:
            printf("伍");
            break;
        case 6:
            printf("陆");
            break;
        case 7:
            printf("柒");
            break;
        case 8:
            printf("捌");
            break;
        case 9:
            printf("玖");
            break;
    }
    if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
        printf("万");
    switch (i4) //千位
    {
        case 1:
            printf("壹仟");
            break;
        case 2:
            printf("贰仟");
            break;
        case 3:
            printf("叁仟");
            break;
        case 4:
            printf("肆仟");
            break;
        case 5:
            printf("伍仟");
            break;
        case 6:
            printf("陆仟");
            break;
        case 7:
            printf("柒仟");
            break;
        case 8:
            printf("捌仟");
            break;
        case 9:
            printf("玖仟");
            break;
    }
    if (!(i4 || i3 || i2 || i1))
        zero_2_exist = 1;
    if (!zero_2_exist && !i4 && num >= 1000)
    {
        printf("零");
        zero_2_exist = 1;
    }
    switch (i3) //百位
    {
        case 1:
            printf("壹佰");
            break;
        case 2:
            printf("贰佰");
            break;
        case 3:
            printf("叁佰");
            break;
        case 4:
            printf("肆佰");
            break;
        case 5:
            printf("伍佰");
            break;
        case 6:
            printf("陆佰");
            break;
        case 7:
            printf("柒佰");
            break;
        case 8:
            printf("捌佰");
            break;
        case 9:
            printf("玖佰");
            break;
    }
    if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
    {
        printf("零");
        zero_2_exist = 1;
    }
    switch (i2) //十位
    {
        case 1:
            printf("壹拾");
            break;
        case 2:
            printf("贰拾");
            break;
        case 3:
            printf("叁拾");
            break;
        case 4:
            printf("肆拾");
            break;
        case 5:
            printf("伍拾");
            break;
        case 6:
            printf("陆拾");
            break;
        case 7:
            printf("柒拾");
            break;
        case 8:
            printf("捌拾");
            break;
        case 9:
            printf("玖拾");
            break;
    }
    if (!i2 && i3 && i1)
    {
        printf("零");
        zero_2_exist = 1;
    }
    switch (i1) //圆
    {
        case 1:
            printf("壹");
            break;
        case 2:
            printf("贰");
            break;
        case 3:
            printf("叁");
            break;
        case 4:
            printf("肆");
            break;
        case 5:
            printf("伍");
            break;
        case 6:
            printf("陆");
            break;
        case 7:
            printf("柒");
            break;
        case 8:
            printf("捌");
            break;
        case 9:
            printf("玖");
            break;
    }
    if (num >= 1)
        printf("圆");
    if (i_1 == 0 && i_2 == 0 && num >= 1)
    {
        printf("整");
        zheng_exist = 1;
    }
    switch (i_1) //角
    {
        case 1:
            printf("壹角");
            break;
        case 2:
            printf("贰角");
            break;
        case 3:
            printf("叁角");
            break;
        case 4:
            printf("肆角");
            break;
        case 5:
            printf("伍角");
            break;
        case 6:
            printf("陆角");
            break;
        case 7:
            printf("柒角");
            break;
        case 8:
            printf("捌角");
            break;
        case 9:
            printf("玖角");
            break;
        default:
            if (i_2 != 0 && num >= 0.1)
                printf("零");
            break;
    }
    if (i_2 == 0 && num >= 0.1 && !zheng_exist)
        printf("整");
    switch (i_2) //分
    {
        case 1:
            printf("壹");
            break;
        case 2:
            printf("贰");
            break;
        case 3:
            printf("叁");
            break;
        case 4:
            printf("肆");
            break;
        case 5:
            printf("伍");
            break;
        case 6:
            printf("陆");
            break;
        case 7:
            printf("柒");
            break;
        case 8:
            printf("捌");
            break;
        case 9:
            printf("玖");
            break;
    }
    if (i_2 != 0)
        printf("分");
    if (num == 0)
        printf("零");
    printf("\n");
    return 0;
}