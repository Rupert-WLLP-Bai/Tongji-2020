/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;
int main()
{
    int fifty, twenty, ten, five, one, point_five, point_one, cent_five, cent_two, cent_one;
    double x;
    double x_decimal_part;
    int count;
    cout << "请输入找零值 : " << endl;
    cin >> x;

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
    cout << "共" << count << "张找零，具体如下 : " << endl;
    if (fifty != 0)
        cout << "50元 : " << fifty << "张" << endl;
    if (twenty != 0)
        cout << "20元 : " << twenty << "张" << endl;
    if (ten != 0)
        cout << "10元 : " << ten << "张" << endl;
    if (five != 0)
        cout << "5元  : " << five << "张" << endl;
    if (one != 0)
        cout << "1元  : " << one << "张" << endl;
    if (point_five != 0)
        cout << "5角  : " << point_five << "张" << endl;
    if (point_one != 0)
        cout << "1角  : " << point_one << "张" << endl;
    if (cent_five != 0)
        cout << "5分  : " << cent_five << "张" << endl;
    if (cent_two != 0)
        cout << "2分  : " << cent_two << "张" << endl;
    if (cent_one != 0)
        cout << "1分  : " << cent_one << "张" << endl;

    return 0;
}

//Done.