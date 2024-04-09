import org.apache.bcel.Const;
import org.apache.bcel.generic.*;
import org.apache.bcel.classfile.*;

public class BCELMainGenerator {

    public static void main(String[] args) {
        try {
            generateMainClass("Main", args);
            System.out.println("Main class generated successfully.");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static void generateMainClass(String className, String[] args) throws Exception {
        ClassGen cg = new ClassGen(className, "java.lang.Object", "<generated>", Const.ACC_PUBLIC | Const.ACC_SUPER, null);
        ConstantPoolGen cp = cg.getConstantPool();

        // Create main method
        InstructionList il = new InstructionList();
        MethodGen mg = new MethodGen(Const.ACC_PUBLIC | Const.ACC_STATIC, Type.VOID, new Type[] { new ArrayType(Type.STRING, 1) }, new String[] { "args" }, "main", className, il, cp);

        for (String arg : args) {
            String[] parts = new String[2];
            if (arg != null) {
                parts = arg.split("_");
            }
                
            switch (parts[0]) {
                case "IConst":
                    il.append(new ICONST(Integer.parseInt(parts[1])));
                    break;
                case "ILoad":
                    il.append(new ILOAD(Integer.parseInt(parts[1])));
                    break;
                case "IStore":
                    il.append(new ISTORE(Integer.parseInt(parts[1])));
                    break;
                case "Print":   
                    il.append(new GETSTATIC(cp.addFieldref("java.lang.System", "out", "Ljava/io/PrintStream;")));
                    il.append(new ILOAD(Integer.parseInt(parts[1])));
                    il.append(new INVOKEVIRTUAL(cp.addMethodref("java.io.PrintStream", "println", "(I)V")));
            }
        }

        il.append(new RETURN());
        mg.setMaxStack();
        mg.setMaxLocals();
        cg.addMethod(mg.getMethod());

        // Write class to file
        JavaClass javaClass = cg.getJavaClass();
        javaClass.dump(className + ".class");
    }
}
