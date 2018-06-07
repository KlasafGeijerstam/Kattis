#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <sstream>
#include <unordered_set>
#include <map>
#include <string.h>
using namespace std;

class Var {
public:
    virtual bool get() = 0;
};

class VarVar : public Var {
protected:
    bool* value;
public:
    char ch = 'a';
    VarVar(bool* val) : value(val) {

    }
    bool get() {
        return *value;
    }
};

class TwoOp : public Var {
protected:
    Var* v2;
    Var* v1;
public:
    TwoOp(Var* v2, Var* v1) : v1(v1), v2(v2)
    {
    }
    virtual bool get() = 0;
};

class NOp : public Var {
private:
    Var* val;
public:
    NOp(Var* v) : val(v) {

    }
    bool get() {
        return !val->get();
    }
};

class KOp : public TwoOp {
public:
    KOp(Var* v1, Var* v2) : TwoOp(v1, v2) {

    }
    bool get() {
        return v1->get() && v2->get();
    }
};

class AOp : public TwoOp {
public:
    AOp(Var* v1, Var* v2) : TwoOp(v1, v2) {

    }
    bool get() {
        return v1->get() || v2->get();
    }
};

class COp : public TwoOp {
public:
    COp(Var* v1, Var* v2) : TwoOp(v1, v2) {

    }
    bool get() {
        return !v1->get() || v2->get();
    }
};

class EOp : public TwoOp {
public:
    EOp(Var* v1, Var* v2) : TwoOp(v1, v2) {

    }
    bool get() {
        return v1->get() == v2->get();
    }
};

map<char, bool*> frees;

Var* build(char** str) {
    switch (**str)
    {
    case 'A':
        (*str)++;
        return new AOp(build(str), build(str));
        break;
    case 'K':
        (*str)++;
        return new KOp(build(str), build(str));
        break;
    case 'C':
        (*str)++;
        return new COp(build(str), build(str));
        break;
    case 'E':
        (*str)++;
        return new EOp(build(str), build(str));
        break;
    case 'N':
        (*str)++;
        return new NOp(build(str));
        break;
    default:
        if (frees.find(**str) == frees.end()) {
            frees[**str] = new bool();
        }
        auto v = new VarVar(frees[**str]);
        v->ch = **str;
        (*str)++;
        return v;
        break;
    }
}

int main()
{
    ios_base::sync_with_stdio(false);
    string p;
    while (true) {
        getline(std::cin, p);
        if (p == "0")
            break;

        char* str = strdup(p.c_str());
        auto v = build(&str);

        vector<char> vars;
        auto f = true;
        for (auto& c : frees)
            vars.push_back(c.first);
        for (int i = 0; i < (1 << vars.size()); i++) {
            for (int j = 0; j < vars.size(); j++)
                *frees[vars[j]] = (1 << j) & i;
            if (!v->get()) {
                f = false;
                goto end;
            }
        }
    end:
        cout << (f ? "tautology" : "not") << endl;
    }
}
