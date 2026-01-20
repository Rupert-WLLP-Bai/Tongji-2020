/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;

void f3(double a, double b, double c)//两不等实根
{
	double delta = b * b - 4 * a * c;
	cout << "有两个不等实根:" << endl;
	cout << "x1=" << -b / (2 * a) + sqrt(delta) / (2 * a) << endl;
	cout << "x2=" << -b / (2 * a) - sqrt(delta) / (2 * a) << endl;
}