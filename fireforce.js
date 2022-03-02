const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

/* describe("fireforce", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it("Is initialized!", async () => {
    // Add your test here.
    const program = anchor.workspace.Fireforce;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
}); */

const main = async () => {
  console.log("🚀 Starting test...");
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Fireforce;

  const schoolAccount = anchor.web3.Keypair.generate();
  const studentA = anchor.web3.Keypair.generate();
  const studentB = anchor.web3.Keypair.generate();
  const studentC = anchor.web3.Keypair.generate();
  const teacher = anchor.web3.Keypair.generate();
  const tx = await program.rpc.initialize({
    accounts: {
      schoolAccount: schoolAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [schoolAccount],
  });

  console.log("📝 Your transaction signature", tx);
  let account = await program.account.schoolAccount.fetch(
    schoolAccount.publicKey
  );
  console.log("👀 School Name", account.name);

  //write test for create class
  await program.rpc.createClass(
    [studentA.publicKey, studentB.publicKey, studentC.publicKey],
    teacher.publicKey,
    "SS1",
    [{ day: "Monday", subjects: ["maths", "english"] }],
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("class", account.classes);

  //test adding teacher to school
  let teacherStruct = {
    name: "teacher",
    address: teacher.publicKey,
    classes: class[
      {
      name: "SS1",
      teacher: teacher.publicKey,
      students: [studentA.publicKey, studentB.publicKey, studentC.publicKey],
      timetable: timetable[
        {
        day: "Monday",
        subjects: ["maths", "english"],
        }
      ]
    }
  ],
  };
  await program.rpc.addTeacher(teacherStruct, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("teacher", account.teachers);

  //test adding student to school
  let studentStruct = {
    name: "studenta",
    class: "SS1",
    address: studentA.publicKey,
  };
  await program.rpc.addStudent(studentStruct, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);

  studentStruct = {
    name: "studentb",
    class: "SS1",
    address: studentB.publicKey,
  };
  await program.rpc.addStudent(studentStruct, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  studentStruct = {
    name: "studentc",
    class: "SS1",
    address: studentC.publicKey,
  };
  await program.rpc.addStudent(studentStruct, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("students", account.students);

  //test promoting students
  await program.rpc.promoteStudents(
    [studentA.publicKey, studentB.publicKey],
    "SS2",
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("students after promotion", account.students);
  console.log("classes after promotion", account.classes);

  // assign teacher to class
  await program.rpc.assignTeacher(teacher.publicKey, "SS2", {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("classes after assigning teaceher", account.classes);

  //assign timetable to class
  await program.rpc.assignTimetable(
    [{ day: "Monday", subjects: ["maths", "english"] }],
    "SS2",
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("classes after assigning timetable", account.classes);

  //test suspending students
  await program.rpc.suspendStudent(studentA.publicKey, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("students after suspension", account.students);

  //test rusticating students
  await program.rpc.rusticateStudent(studentA.publicKey, {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("students after rusticating", account.students);
  console.log("classes after rusticating", account.classes);

  //test graduating students
  await program.rpc.graduateStudents([studentB.publicKey, studentC.publicKey], {
    accounts: {
      schoolAccount: schoolAccount.publicKey,
    },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("students after graduating", account.students);
  console.log("classes after graduating", account.classes);

  // test for soft skills
  /* let addSoftSkill = {
      skill: "cleanliness",
      address: teacher.publicKey,
      approved: true,
      date: "March, 2022",
  };
  await program.rpc.addSoftSkill(addSoftSkill, {
      accounts: {
          schoolAccount: schoolAccount.publicKey,
      },
  });
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey); */

  await program.rpc.addSoftSkill(
    [studentA.publicKey, studentB.publicKey, studentC.publicKey],
    teacher.publicKey,
    "March, 2022",
    "Cleanliness",
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("Soft Skill", account.students);


  // test to add grade
  await program.rpc.addGrade(
    [studentA.publicKey, studentB.publicKey, studentC.publicKey],
    teacher.publicKey,
    "maths",
    "march, 2022",
    85,
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("Grade Added", account.students);

  // test for attendance
  await program.rpc.addAttendance(
    [studentA.publicKey, studentB.publicKey, studentC.publicKey],
    teacher.publicKey,
    "march, 2022",
    ["maths", "english"],
    true,
    {
      accounts: {
        schoolAccount: schoolAccount.publicKey,
      },
    }
  );
  account = await program.account.schoolAccount.fetch(schoolAccount.publicKey);
  console.log("Student is present", account.students);

};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();