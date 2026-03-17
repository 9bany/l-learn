#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    int T;
    cin >> T;

    while (T--) {
        long double N;
        cin >> N;

        long double result = 1.0L / N;

        cout << fixed << setprecision(16) << result << endl;
    }

    return 0;
}