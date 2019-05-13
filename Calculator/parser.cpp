#include <memory>
#include "parser.h"
#include <stack>
#include <iostream>
#include <queue>
#include <cctype>

using std::cout;
using std::ostream;
using std::endl;
using std::stack;
using std::queue;
using std::shared_ptr;
using std::string;
using std::cin;

int main() {
    string s;
    getline(cin, s);
    Expression ex(s);
    cout << ex << endl;
    return 0;
}

Expression::Expression(string exp) : exString(exp) {
    queue<shared_ptr<Atom>> q;
    bool empty = true;
    int holder = 0;
    for(auto it = exp.cbegin(); it != exp.cend(); it++) {
        if(isdigit(*it)) {
            holder = holder * 10 + (*it - '0');
            empty = false;
        } else if (!isblank(*it)) {
            q.push(std::make_unique<Val>(holder));
            holder = 0;
            empty = true;
            if(*it == '+') {
                q.push(std::make_unique<Sum>());
            } else if (*it == '-') {
                q.push(std::make_unique<Subtract>());
            } else if (*it == '*') {
                q.push(std::make_unique<Multiply>());
            } else if (*it == '/') {
                q.push(std::make_unique<Divide>());
            }
        }
    }
    if(!empty)
        q.push(std::make_unique<Val>(holder));
    queue<shared_ptr<Atom>> rpn = shuntingYard(std::move(q));
    this->value = evaluateRpn(std::move(rpn));
}

ostream& operator<<(ostream& out, const Expression& ex) {
    return out << "Input: " << ex.exString << "; value: " << ex.value << endl;
}

bool Val::execute(stack<NumberType>& s) const {
    s.push(this->value);
    return true;
}

bool BinOp::execute(stack<NumberType>& s) const {
    if(s.size() < 2)
        return false;

    auto left = s.top();
    s.pop();
    auto right = s.top();
    s.pop();
    s.push(this->run(left, right));
    return true;
}

NumberType Sum::run(NumberType l, NumberType r) const {
    return l + r;
}

NumberType Subtract::run(NumberType l, NumberType r) const {
    return r - l;
}

NumberType Multiply::run(NumberType l, NumberType r) const {
    return l * r;
}

NumberType Divide::run(NumberType l, NumberType r) const {
    return r / l;
}

Val::Val(NumberType i) {
    this->value = i;
}

string Val::print() const {
    return std::to_string(this->value);
}

string Sum::print() const {
    return "+";
}

string Multiply::print() const {
    return "*";
}

string Subtract::print() const {
    return "-";
}

string Divide::print() const {
    return "/";
}

int Val::outfix(std::shared_ptr<Atom> me, std::stack<shared_ptr<BinOp>>& ops, std::queue<shared_ptr<Atom>>& vals) const {
    (void)ops;
    vals.push(me);
    return 0;
}

int BinOp::outfix(std::shared_ptr<Atom> me, std::stack<shared_ptr<BinOp>>& ops, std::queue<shared_ptr<Atom>>& vals) const {
    auto bopMe = std::dynamic_pointer_cast<BinOp>(me);
    while(!ops.empty() && ops.top()->precedence > bopMe->precedence) {
        auto top = ops.top();
        ops.pop();
        vals.push(top);
    }
    ops.push(std::dynamic_pointer_cast<BinOp>(me));
    return 0;
}

queue<shared_ptr<Atom>> shuntingYard(queue<shared_ptr<Atom>> q) {
    std::stack<shared_ptr<BinOp>> ops;
    std::queue<shared_ptr<Atom>> vals;
    while(!q.empty()) {
        auto t = q.front();
        q.pop();
        t->outfix(t, ops, vals);
    }
    while(!ops.empty()) {
        vals.push(ops.top());
        ops.pop();
    }
    return vals;
}

NumberType evaluateRpn(queue<shared_ptr<Atom>> q) {
    stack<NumberType> other;
    while(!q.empty()) {
        auto worked = q.front()->execute(other);
        if(!worked)
            throw "Ran out of numbers!";
        q.pop();
    }
    if(other.empty())
        throw "Ran out of numbers!";
    else
        return other.top();
}
