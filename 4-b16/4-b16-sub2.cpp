/*2052526 信15 白俊豪*/
#include <iostream>
#include <cmath>
using namespace std;

void f2(double a, double b, double c)	//两共轭复根
{
	double delta = b * b - 4 * a * c;
	cout << "有两个虚根:" << endl;
	if (fabs(b) < 1e-6)//无实数部分
	{
		if (a >= 1e-6)
		{
			if (fabs((sqrt(-delta) / (2 * a) - 1)) >= 1e-6)
			{
				cout << "x1=" << sqrt(-delta) / (2 * a) << "i" << endl;
				cout << "x2=" << -sqrt(-delta) / (2 * a) << "i" << endl;
			}
			else
			{
				cout << "x1=" << "i" << endl;
				cout << "x2=" << "-i" << endl;
			}
		}
		else
		{
			if (fabs((sqrt(-delta) / (2 * a) + 1)) >= 1e-6)
			{
				cout << "x1=" << sqrt(-delta) / (2 * a) << "i" << endl;
				cout << "x2=" << -sqrt(-delta) / (2 * a) << "i" << endl;
			}
			else
			{
				cout << "x1=" << "i" << endl;
				cout << "x2=" << "-i" << endl;
			}
		}
	}
	else//有实数部分
	{
		if (a >= 1e-6)
		{
			if (fabs((sqrt(-delta) / (2 * a) - 1)) >= 1e-6)
			{
				cout << "x1=" << -b / (2 * a) << "+" << sqrt(-delta) / (2 * a) << "i" << endl;
				cout << "x2=" << -b / (2 * a) << -sqrt(-delta) / (2 * a) << "i" << endl;
			}
			else
			{
				cout << "x1=" << -b / (2 * a) << "+i" << endl;
				cout << "x2=" << -b / (2 * a) << "-i" << endl;
			}
		}
		else
		{
			if (fabs((sqrt(-delta) / (2 * a) + 1)) >= 1e-6)
			{
				cout << "x1=" << -b / (2 * a) << sqrt(-delta) / (2 * a) << "i" << endl;
				cout << "x2=" << -b / (2 * a) << "+" << -sqrt(-delta) / (2 * a) << "i" << endl;
			}
			else
			{
				cout << "x1=" << -b / (2 * a) << "+i" << endl;
				cout << "x2=" << -b / (2 * a) << "-i" << endl;
			}
		}

	}
}
