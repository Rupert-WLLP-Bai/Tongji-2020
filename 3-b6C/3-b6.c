/*2052526 信15 白俊豪*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
int main()
{
    int year, month, day;
    int order;
    int is_leap_year = 0;
    printf("请输入年,月,日 : \n");
    scanf("%d%d%d", &year, &month, &day);
    //判断闰年
    if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
        is_leap_year = 1;
    //判断月份,月份与日期的关系是否合法\n
    if (is_leap_year == 1) // leap year
    {
        switch (month)
        {
            case 1:
                if (day >= 1 && day <= 31)
                {
                    order = day;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;

            case 2:
                if (day >= 1 && day <= 29)
                {
                    order = day + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 3:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 4:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 29 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 5:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 6:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 29 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 7:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 8:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 9:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 10:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 11:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 12:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            default:
                printf("月份不正确");
                break;
        }
    }
    else //not leap year
    {
        switch (month)
        {
            case 1:
                if (day >= 1 && day <= 31)
                {
                    order = day;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;

            case 2:
                if (day >= 1 && day <= 28)
                {
                    order = day + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 3:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 4:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 28 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 5:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 6:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 28 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 7:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 8:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 9:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 10:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 11:
                if (day >= 1 && day <= 30)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            case 12:
                if (day >= 1 && day <= 31)
                {
                    order = day + 31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30;
                    printf("%d-%d-%d是%d年的第%d天\n", year, month, day, year, order);
                }
                else
                {
                    printf("输入错误-日与月的关系非法\n");
                }
                break;
            default:
                printf("月份不正确");
                break;
        }
    }
    return 0;
}

//Done.
