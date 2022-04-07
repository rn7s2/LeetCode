package p50;

public class Solution {
    public double myPow(double x, int n) {
        double ret = 1.0;
        boolean flag = false;
        long b = n;

        if (b == 0)
            return ret;

        // int: [-2^31, 2^31-1)
        // -b might still be -2^31
        // use long to bypass that issue
        if (b < 0) {
            flag = true;
            b = -b;
        }

        while (b > 0) {
            if (b % 2 == 1)
                ret = ret * x;
            x = x * x;
            b = b / 2;
        }

        if(flag)
            ret = 1.0 / ret;

        return ret;
    }
}
