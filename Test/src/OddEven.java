import javax.swing.JOptionPane;
 
public class OddEven {
 /**
 * "input" is the number that the user gives to the computer
 */
 private int input; // a whole number("int" means integer)
 
 /**
 * This is the constructor method. It gets called when an object of the OddEven type
 * is being created.
 */
 public OddEven() {
 /*
 * In most Java programs constructors can initialize objects with default values, or create
 * other objects that this object might use to perform its functions. In some Java programs, the
 * constructor may simply be an empty function if nothing needs to be initialized prior to the
 * functioning of the object. In this program's case, an empty constructor would suffice.
 * A constructor must exist; however, if the user doesn't put one in then the compiler
 * will create an empty one.
 */
 }
 
 /**
 * This is the main method. It gets called when this class is run through a Java interpreter.
 * @param args command line arguments (unused)
 */
 public static void main(final String[] args) {
 /*
 * This line of code creates a new instance of this class called "number" (also known as an
 * Object) and initializes it by calling the constructor. The next line of code calls
 * the "showDialog()" method, which brings up a prompt to ask you for a number
 */
 OddEven number = new OddEven();
 number.showDialog();
 }
 
 public void showDialog() {
 /*
 * "try" makes sure nothing goes wrong. If something does,
 * the interpreter skips to "catch" to see what it should do.
 */
 try {
 /*
 * The code below brings up a JOptionPane, which is a dialog box
 * The String returned by the "showInputDialog()" method is converted into
 * an integer, making the program treat it as a number instead of a word.
 * After that, this method calls a second method, calculate() that will
 * display either "Even" or "Odd."
 */
 this.input = Integer.parseInt(JOptionPane.showInputDialog("Please enter a number."));
 this.calculate();
 } catch (final NumberFormatException e) {
 /*
 * Getting in the catch block means that there was a problem with the format of
 * the number. Probably some letters were typed in instead of a number.
 */
 System.err.println("ERROR: Invalid input. Please type in a numerical value.");
 }
 }
 
 /**
 * When this gets called, it sends a message to the interpreter.
 * The interpreter usually shows it on the command prompt (For Windows users)
 * or the terminal (For *nix users).(Assuming it's open)
 */
 private void calculate() {
 if ((this.input % 2) == 0) {
 JOptionPane.showMessageDialog(null, "Even");
 } else {
 JOptionPane.showMessageDialog(null, "Odd");
 }
 }
}