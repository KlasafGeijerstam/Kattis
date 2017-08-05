#include <stdio.h>
#include <stdlib.h>

typedef struct f {
    int* dat;
    long length;
}fenwick;

//Creates a new 1-indexed fenwick tree of specified size
    fenwick* fen(int size)
    {
        fenwick* f = (fenwick*)malloc(sizeof(fenwick));
        f->dat = (int*)calloc(size + 1, sizeof(int));
        f->length = size + 1;
        return f;
    }

    //Increments the value of index by value
    void increment(fenwick* f,int index, int value)
    {
        while (index < f->length) {
            f->dat[index] += value;
            index += index & (-index);
        }
    }

    //Returns the value from index to 0
    int sumf(fenwick* f,int index)
    {
        int sum = 0;
        while (index > 0) {
            sum += f->dat[index];
            index -= index & (-index);
        }
        return sum;
    }

    int range(fenwick* f,int i1, int i2)
    {
        return sumf(f,i2) - sumf(f,i1 - 1);
    }

int main()
{
    long n, k;
    scanf("%ld %ld",&n,&k);
    fenwick* f = fen(n);
    char* val = (char*)calloc(n + 1,sizeof(char));
    char v;
    long l, r;
    for (int i = 0; i < k; i++)
    {
        scanf(" %c",&v);
        if (v == 'F') 
        {
            scanf("%ld",&l);
            if (val[l]) {
                increment(f,l, -1);
                val[l] = 0;
            }
            else {
                increment(f,l, 1);
                val[l] = 1;
            }
        }
        else 
        {
            scanf("%ld %ld",&l,&r);
            printf("%l\n", range(f,l, r));
        }
    }
}
