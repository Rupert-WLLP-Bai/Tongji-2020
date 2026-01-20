/*2052526 ÐÅ15 °×¿¡ºÀ*/
#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <math.h>
#include <stdbool.h>
int main()
{
    int i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i_1, i_2, temp1, temp2, temp3, temp4, temp5, temp6, temp7, temp8, temp9;
    double temp_1, num, num_int, num_decimal_part;
    bool i10_19_exist = 1, zero_1_exist = 0, zero_2_exist = 0, zheng_exist = 0;
    printf("ÇëÊäÈë[0-100ÒÚ)Ö®¼äµÄÊý×Ö,Ð¡Êýµãºó×î¶àÁ½Î»£º\n");
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
    switch (i10) //Ê®ÒÚÎ»
    {
        case 1:
            printf("Ò¼Ê°");
            break;
        case 2:
            printf("·¡Ê°");
            break;
        case 3:
            printf("ÈþÊ°");
            break;
        case 4:
            printf("ËÁÊ°");
            break;
        case 5:
            printf("ÎéÊ°");
            break;
        case 6:
            printf("Â½Ê°");
            break;
        case 7:
            printf("ÆâÊ°");
            break;
        case 8:
            printf("°ÆÊ°");
            break;
        case 9:
            printf("¾ÁÊ°");
            break;
    }
    switch (i9) //ÒÚÎ»
    {
        case 1:
            printf("Ò¼");
            break;
        case 2:
            printf("·¡");
            break;
        case 3:
            printf("Èþ");
            break;
        case 4:
            printf("ËÁ");
            break;
        case 5:
            printf("Îé");
            break;
        case 6:
            printf("Â½");
            break;
        case 7:
            printf("Æâ");
            break;
        case 8:
            printf("°Æ");
            break;
        case 9:
            printf("¾Á");
            break;
    }
    if (i10 != 0 || i9 != 0)
        printf("ÒÚ");
    else
        i10_19_exist = 0;
    switch (i8) //Ç§ÍòÎ»
    {
        case 1:
            printf("Ò¼Çª");
            break;
        case 2:
            printf("·¡Çª");
            break;
        case 3:
            printf("ÈþÇª");
            break;
        case 4:
            printf("ËÁÇª");
            break;
        case 5:
            printf("ÎéÇª");
            break;
        case 6:
            printf("Â½Çª");
            break;
        case 7:
            printf("ÆâÇª");
            break;
        case 8:
            printf("°ÆÊ°");
            break;
        case 9:
            printf("¾ÁÊ°");
            break;
    }
    if (!(i8 || i7 || i6 || i5))
        zero_1_exist = 1;
    if (!zero_1_exist && i10_19_exist && !i8)
    {
        printf("Áã");
        zero_1_exist = 1;
    }
    switch (i7) //°ÙÍòÎ»
    {
        case 1:
            printf("Ò¼°Û");
            break;
        case 2:
            printf("·¡°Û");
            break;
        case 3:
            printf("Èþ°Û");
            break;
        case 4:
            printf("ËÁ°Û");
            break;
        case 5:
            printf("Îé°Û");
            break;
        case 6:
            printf("Â½°Û");
            break;
        case 7:
            printf("Æâ°Û");
            break;
        case 8:
            printf("°Æ°Û");
            break;
        case 9:
            printf("¾Á°Û");
            break;
    }
    if (!zero_1_exist && !i7 && i6 && num >= 1000000)
    {
        printf("Áã");
        zero_1_exist = 1;
    }
    switch (i6) //Ê®ÍòÎ»
    {
        case 1:
            printf("Ò¼Ê°");
            break;
        case 2:
            printf("·¡Ê°");
            break;
        case 3:
            printf("ÈþÊ°");
            break;
        case 4:
            printf("ËÁÊ°");
            break;
        case 5:
            printf("ÎéÊ°");
            break;
        case 6:
            printf("Â½Ê°");
            break;
        case 7:
            printf("ÆâÊ°");
            break;
        case 8:
            printf("°ÆÊ°");
            break;
        case 9:
            printf("¾ÁÊ°");
            break;
    }
    if (i7 && !i6 && i5 && num >= 100000)
    {
        printf("Áã");
        zero_1_exist = 1;
    }
    switch (i5) //ÍòÎ»
    {
        case 1:
            printf("Ò¼");
            break;
        case 2:
            printf("·¡");
            break;
        case 3:
            printf("Èþ");
            break;
        case 4:
            printf("ËÁ");
            break;
        case 5:
            printf("Îé");
            break;
        case 6:
            printf("Â½");
            break;
        case 7:
            printf("Æâ");
            break;
        case 8:
            printf("°Æ");
            break;
        case 9:
            printf("¾Á");
            break;
    }
    if (i5 != 0 || i6 != 0 || i7 != 0 || i8 != 0)
        printf("Íò");
    switch (i4) //Ç§Î»
    {
        case 1:
            printf("Ò¼Çª");
            break;
        case 2:
            printf("·¡Çª");
            break;
        case 3:
            printf("ÈþÇª");
            break;
        case 4:
            printf("ËÁÇª");
            break;
        case 5:
            printf("ÎéÇª");
            break;
        case 6:
            printf("Â½Çª");
            break;
        case 7:
            printf("ÆâÇª");
            break;
        case 8:
            printf("°ÆÇª");
            break;
        case 9:
            printf("¾ÁÇª");
            break;
    }
    if (!(i4 || i3 || i2 || i1))
        zero_2_exist = 1;
    if (!zero_2_exist && !i4 && num >= 1000)
    {
        printf("Áã");
        zero_2_exist = 1;
    }
    switch (i3) //°ÙÎ»
    {
        case 1:
            printf("Ò¼°Û");
            break;
        case 2:
            printf("·¡°Û");
            break;
        case 3:
            printf("Èþ°Û");
            break;
        case 4:
            printf("ËÁ°Û");
            break;
        case 5:
            printf("Îé°Û");
            break;
        case 6:
            printf("Â½°Û");
            break;
        case 7:
            printf("Æâ°Û");
            break;
        case 8:
            printf("°Æ°Û");
            break;
        case 9:
            printf("¾Á°Û");
            break;
    }
    if (!zero_2_exist && !i3 && (i2 || i1) && num >= 100)
    {
        printf("Áã");
        zero_2_exist = 1;
    }
    switch (i2) //Ê®Î»
    {
        case 1:
            printf("Ò¼Ê°");
            break;
        case 2:
            printf("·¡Ê°");
            break;
        case 3:
            printf("ÈþÊ°");
            break;
        case 4:
            printf("ËÁÊ°");
            break;
        case 5:
            printf("ÎéÊ°");
            break;
        case 6:
            printf("Â½Ê°");
            break;
        case 7:
            printf("ÆâÊ°");
            break;
        case 8:
            printf("°ÆÊ°");
            break;
        case 9:
            printf("¾ÁÊ°");
            break;
    }
    if (!i2 && i3 && i1)
    {
        printf("Áã");
        zero_2_exist = 1;
    }
    switch (i1) //Ô²
    {
        case 1:
            printf("Ò¼");
            break;
        case 2:
            printf("·¡");
            break;
        case 3:
            printf("Èþ");
            break;
        case 4:
            printf("ËÁ");
            break;
        case 5:
            printf("Îé");
            break;
        case 6:
            printf("Â½");
            break;
        case 7:
            printf("Æâ");
            break;
        case 8:
            printf("°Æ");
            break;
        case 9:
            printf("¾Á");
            break;
    }
    if (num >= 1)
        printf("Ô²");
    if (i_1 == 0 && i_2 == 0 && num >= 1)
    {
        printf("Õû");
        zheng_exist = 1;
    }
    switch (i_1) //½Ç
    {
        case 1:
            printf("Ò¼½Ç");
            break;
        case 2:
            printf("·¡½Ç");
            break;
        case 3:
            printf("Èþ½Ç");
            break;
        case 4:
            printf("ËÁ½Ç");
            break;
        case 5:
            printf("Îé½Ç");
            break;
        case 6:
            printf("Â½½Ç");
            break;
        case 7:
            printf("Æâ½Ç");
            break;
        case 8:
            printf("°Æ½Ç");
            break;
        case 9:
            printf("¾Á½Ç");
            break;
        default:
            if (i_2 != 0 && num >= 0.1)
                printf("Áã");
            break;
    }
    if (i_2 == 0 && num >= 0.1 && !zheng_exist)
        printf("Õû");
    switch (i_2) //·Ö
    {
        case 1:
            printf("Ò¼");
            break;
        case 2:
            printf("·¡");
            break;
        case 3:
            printf("Èþ");
            break;
        case 4:
            printf("ËÁ");
            break;
        case 5:
            printf("Îé");
            break;
        case 6:
            printf("Â½");
            break;
        case 7:
            printf("Æâ");
            break;
        case 8:
            printf("°Æ");
            break;
        case 9:
            printf("¾Á");
            break;
    }
    if (i_2 != 0)
        printf("·Ö");
    if (num == 0)
        printf("Áã");
    printf("\n");
    return 0;
}