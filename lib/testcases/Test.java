class Test {
public static void main(String[] args) {
SetterGetter m = new SetterGetter();
System.out.println(m.getX());
m.setX(5);
m.setX(8);
m.setX(257);
m.setX(0);
m.setX(69);
System.out.println(m.getB());
m.setB(true);
m.setB(false);
m.setB(true);
m.setB(false);
m.setB(true);
System.out.println(m.getC());
m.setC('c');
m.setC('x');
m.setC('!');
m.setC('a');
m.setC('f');
System.out.println(m.getS());
m.setS("a");
m.setS("test");
m.setS("Hello World!?!");
m.setS("A bit of escaping going on here... \"");
m.setS("Just another string test");
}}