/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <math.h>
int main()
{
    int fifty, twenty, ten, five, one, point_five, point_one, cent_five, cent_two, cent_one;
    double x;
    double x_decimal_part;
    int count;
    printf("请输入找零值 : \n");
    scanf("%lf", &x);

    fifty = (int)(x / 50);
    twenty = (int)((x - fifty * 50) / 20);
    ten = (int)((x - 50 * fifty - 20 * twenty) / 10);
    five = (int)((x - 50 * fifty - 20 * twenty - 10 * ten) / 5);
    one = (int)((x - 50 * fifty - 20 * twenty - 10 * ten - 5 * five) / 1);

    x_decimal_part = fmod(x, 1);
    point_five = (int)(x_decimal_part / 0.5);
    point_one = (int)((x_decimal_part - 0.5 * point_five) / 0.1);
    cent_five = (int)((x_decimal_part - 0.5 * point_five - 0.1 * point_one) / 0.05);
    cent_two = (int)((x_decimal_part - 0.5 * point_five - 0.1 * point_one - 0.05 * cent_five) / 0.02);
    cent_one = (int)((x_decimal_part - 0.5 * point_five - 0.1 * point_one - 0.05 * cent_five - 0.02 * cent_two + 1e-5) / 0.01);

    count = fifty + twenty + ten + five + one + point_five + point_one + cent_five + cent_two + cent_one;
    printf("共%d张找零，具体如下 : \n", count);
    if (fifty != 0)
        printf("50元 : %d张\n", fifty);
    if (twenty != 0)
        printf("20元 : %d张\n", twenty);
    if (ten != 0)
        printf("10元 : %d张\n", ten);
    if (five != 0)
        printf("5元  : %d张\n", five);
    if (one != 0)
        printf("1元  : %d张\n", one);
    if (point_five != 0)
        printf("5角  : %d张\n", point_five);
    if (point_one != 0)
        printf("1角  : %d张\n", point_one);
    if (cent_five != 0)
        printf("5分  : %d张\n", cent_five);
    if (cent_two != 0)
        printf("2分  : %d张\n", cent_two);
    if (cent_one != 0)
        printf("1分  : %d张\n", cent_one);

    return 0;
}

//Done.