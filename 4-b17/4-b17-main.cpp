/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;

void f1();
void f2();
void f3();
void f4();
double a, b, c, delta;

int main()
{
	cout << "请输入一元二次方程的三个系数a,b,c" << endl;
	cin >> a >> b >> c;
	if (fabs(a) < 1e-6)//非二次方程
		f1();
	else//二次方程
	{
		delta = b * b - 4 * a * c;
		if (delta < 0)//无实根
			f2();
		else if (fabs(delta) > 1e-6)//两不等实根
			f3();
		else//两相等实根
			f4();
	}
	return 0;
}