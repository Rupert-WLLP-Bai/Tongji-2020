/*2052526 信15 白俊豪*/
#include<iostream>
#include<cmath>
using namespace std;

double cross_product(double x1, double y1, double x2, double y2, double x3, double y3)
{
	double delta_x1 = x2 - x1;
	double delta_y1 = y2 - y1;
	double delta_x2 = x3 - x2;
	double delta_y2 = y3 - y2;
	return delta_x1 * delta_y2 - delta_x2 * delta_y1;
}

bool convex(int num,double x[8],double y[8])
{
	bool flag = 1;
	for (int i = 0; i < num; i++)
	{
		/*cout << "第" << i+1 << "组 : " << cross_product(x[i % num], y[i % num], x[(i + 1) % num], y[(i + 1) % num], x[(i + 2) % num], y[(i + 2) % num])<< endl;
		cout << cross_product(x[i % num], y[i % num], x[(i + 1) % num], y[(i + 1) % num], x[(i + 2) % num], y[(i + 2) % num]) * cross_product(x[(i + 1) % num], y[(i + 1) % num], x[(i + 2) % num], y[(i + 2) % num], x[(i + 3) % num], y[(i + 3) % num]) << endl;*/
		
		if ( cross_product(x[i % num], y[i % num], x[(i + 1) % num], y[(i + 1) % num], x[(i + 2) % num], y[(i + 2) % num]) * cross_product(x[(i + 1) % num], y[(i + 1) % num], x[(i + 2) % num], y[(i + 2) % num], x[(i + 3) % num], y[(i + 3) % num])<= 0)
			return 0;
	}

	return flag;
}

double area_calculate(int num, double x[8], double y[8])
{
	double S = 0;
	for (int i = 0; i < num - 2; i++)
		S += fabs(cross_product(x[0], y[0], x[i + 1], y[i + 1], x[i + 2], y[i + 2])) / 2.0;
	return S;
}

int main()
{
	double x[8] = { 0 };
	double y[8] = { 0 };
	int num;//顶点数
	double x0, y0;

	while (1)//输入顶点数量
	{
		cout << "请输入多边形的顶点数量(4-7)" << endl;
		cin >> num;
		if (!cin.fail() && num >= 4 && num <= 7)
			break;
		else
		{
			cin.clear();
			cin.ignore(1024, '\n');
		}
	}

	for (int i = 0; i < num; i++)//输入顶点坐标
	{
		while (1)
		{
			cout << "请输入第" << i + 1 << "个顶点的坐标" << endl;
			cin >> x0 >> y0;
			if (!cin.fail())
			{
				x[i] = x0;
				y[i] = y0;
				break;
			}
			else
			{
				cin.clear();
				cin.ignore(1024, '\n');
			}
		}
	}

	if (convex(num,x,y))
		cout << "凸" << num << "边形的面积为" << area_calculate(num,x,y) << endl;
	else
		cout << "不是凸" << num << "边形" << endl;

	return 0;
}