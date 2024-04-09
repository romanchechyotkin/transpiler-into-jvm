import org.apache.bcel.Const;
import org.apache.bcel.generic.*;
import org.apache.bcel.classfile.*;

public class BCELMainGenerator {

    public static void main(String[] args) {
        System.out.println(args.length);

        try {
            generateMainClass("Main");
            System.out.println("Main class generated successfully.");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static void generateMainClass(String className) throws Exception {
        ClassGen cg = new ClassGen(className, "java.lang.Object", "<generated>", Const.ACC_PUBLIC | Const.ACC_SUPER, null);
        ConstantPoolGen cp = cg.getConstantPool();

        // Create main method
        InstructionList il = new InstructionList();
        MethodGen mg = new MethodGen(Const.ACC_PUBLIC | Const.ACC_STATIC, Type.VOID, new Type[] { new ArrayType(Type.STRING, 1) }, new String[] { "args" }, "main", className, il, cp);

        il.append(new GETSTATIC(cp.addFieldref("java.lang.System", "out", "Ljava/io/PrintStream;")));
        il.append(new LDC(cp.addString("Hello, World!")));
        il.append(new INVOKEVIRTUAL(cp.addMethodref("java.io.PrintStream", "println", "(Ljava/lang/String;)V")));
        il.append(new RETURN());

        mg.setMaxStack();
        mg.setMaxLocals();
        cg.addMethod(mg.getMethod());

        // Write class to file
        JavaClass javaClass = cg.getJavaClass();
        javaClass.dump(className + ".class");
    }
}
