struct fenwick {
    long* dat;
    long length;

    //Creates a new 1-indexed fenwick tree of specified size
    fenwick(long size)
    {
        dat = (long*)calloc(size + 1, sizeof(long));
        length = size + 1;
    }

    //Increments the value of index by value
    void increment(long index, long value)
    {
        while (index < length) {
            dat[index] += value;
            index += index & (-index);
        }
    }

    //Sets the value of index and all subsequent positions to index2 to 0
    void zero(long index, long index2)
    {
        while (index <= index2) {
            dat[index] = 0;
            index += index & (-index);
        }
    }

    //Returns the value from index to 0
    long sum(long index)
    {
        long sum = 0;
        while (index > 0) {
            sum += dat[index];
            index -= index & (-index);
        }
        return sum;
    }

    long range(long i1, long i2)
    {
        return sum(i2) - sum(i1 - 1);
    }

    long get(long index)
    {
        return dat[index];
    }

    //Sets index to value
    void set(long index, long value)
    {
        dat[index] = value;
    }

    ~fenwick()
    {
        free(dat);
    }
};

