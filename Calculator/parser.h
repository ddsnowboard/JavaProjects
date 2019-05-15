#pragma once
#include <queue>
#include <unordered_map>
#include <optional>
#include <stack>
#include <string>

typedef int NumberType;

class BinOp;

class Expression {
    private:
        std::string exString;
        NumberType value;
        bool failed = false;
    public:
        Expression(std::string exp);
        Expression(std::string exp, const std::unordered_map<std::string, NumberType>& varEnv);
        bool succeeded() const;
        friend std::ostream& operator<<(std::ostream& out, const Expression& ex);
};

class Atom {
    public: 
        virtual bool execute(std::stack<NumberType>& s) const = 0;
        virtual int outfix(std::shared_ptr<Atom> me, std::stack<std::shared_ptr<BinOp>>& ops, std::queue<std::shared_ptr<Atom>>& vals) const = 0;
        virtual std::string print() const = 0;
};

class Val : public Atom {
    NumberType value;
    public: 
    virtual bool execute(std::stack<NumberType>& s) const;
    virtual std::string print() const;
    virtual int outfix(std::shared_ptr<Atom> me, std::stack<std::shared_ptr<BinOp>>& ops, std::queue<std::shared_ptr<Atom>>& vals) const;
    Val(NumberType i);
};

class BinOp : public Atom {
    public:
        virtual bool execute(std::stack<NumberType>& s) const;
        virtual std::string print() const = 0;
        virtual int outfix(std::shared_ptr<Atom> me, std::stack<std::shared_ptr<BinOp>>& ops, std::queue<std::shared_ptr<Atom>>& vals) const;
        const int precedence = 0;
    protected: 
        virtual NumberType run(NumberType l, NumberType r) const = 0;
};

class Sum : public BinOp {
    private:
        virtual NumberType run(NumberType l, NumberType r) const;
    public:
        const int precedence = 1;
        virtual std::string print() const;
};

class Multiply : public BinOp {
    private:
        virtual NumberType run(NumberType l, NumberType r) const;
    public:
        const int precedence = 2;
        virtual std::string print() const;
};

class Subtract : public BinOp {
    private:
        virtual NumberType run(NumberType l, NumberType r) const;
    public:
        const int precedence = 1;
        virtual std::string print() const;
};

class Divide : public BinOp {
    private:
        virtual NumberType run(NumberType l, NumberType r) const;
    public:
        const int precedence = 2;
        virtual std::string print() const;
};

std::queue<std::shared_ptr<Atom>> shuntingYard(std::queue<std::shared_ptr<Atom>> q);

std::optional<NumberType> evaluateRpn(std::queue<std::shared_ptr<Atom>> q);

bool consumeNumber(std::string::const_iterator& it, const std::string::const_iterator& end, int& out);

bool consumeVar(std::string::const_iterator& it, const std::string::const_iterator& end, std::string& out);
