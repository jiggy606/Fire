use std::vec;
use anchor_lang::prelude::*;
// use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod fireforce {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //school initialization 
        let school_account = &mut ctx.accounts.school_account;
        school_account.name = String::from("ISL");
       // school_account.number_of_classes = 6;
        Ok(())
    }

    ///assigns students, teachers and timetable to a class
    pub fn create_class(ctx: Context<CreateClass>, student_addresses: Vec<Pubkey>, teacher_address: Pubkey, class_name: String, time_table: Vec<TimetableStruct>)-> Result<()>{
        let  school_account= &mut ctx.accounts.school_account;
        let class = ClassStruct{
            name: class_name,
            teacher: teacher_address,
            students: student_addresses,
            time_table,
        };
        school_account.classes.push(class);
        Ok(())
    }
 ///assigns  teacher alone to class
 pub fn assign_teacher(ctx: Context<AssignTeacher>, teacher_address: Pubkey, class_name: String)-> Result<()>{
    let  school_account= &mut ctx.accounts.school_account;
    school_account.classes.iter_mut().find(|class| class.name == class_name).unwrap().teacher= teacher_address;
    Ok(())

 }
 ///assigns  timetable alone to class
 pub fn assign_timetable(ctx: Context<AssignTimetable>, timetable: Vec<TimetableStruct>, class_name: String)-> Result<()>{
    let  school_account= &mut ctx.accounts.school_account;
    school_account.classes.iter_mut().find(|class| class.name == class_name).unwrap().time_table= timetable;
    Ok(())

 }
    ///promote MULTIPLE students in one class to next class 
    pub fn promote_students(ctx: Context<PromoteStudent>, student_addresses:Vec<Pubkey>, class_name: String) -> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        let mut new_class = ClassStruct{
          name: class_name.to_string(),
          teacher: Pubkey::default(),
          students: vec![],
          time_table: vec![]
        };
        //find the index of class_name in classes if it exists else add it
        for student in &mut student_addresses.iter(){
          let student_index =school_account.students.iter().position(|x| x.address == *student).unwrap();
          let student_class_index=school_account.classes.iter().position(|x| x.name==school_account.students[student_index].class).unwrap();
          //check if class_name is in classes, i.e class exists. if not, add it
          if school_account.classes[student_class_index].name==class_name{
            school_account.classes[student_class_index].students.push(*student);
          }
          else{
            //create new class
            let mut students= vec![];
            students.push(*student);
               new_class = ClassStruct{
              name: class_name.to_string(),
              teacher: Pubkey::default(),
              students: students,
              time_table: vec![]
            };
          }
          //update student struct
          school_account.students[student_index].class=class_name.to_string();
          //remove student from old class
          school_account.classes[student_class_index].students.remove(student_index);
        }
        school_account.classes.push(new_class);
       

      Ok(())
    } 
     
    ///add teacher to the school
     pub fn add_teacher(ctx: Context<AddTeacher>, teacher_struct: TeacherStruct)-> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        school_account.teachers.push(teacher_struct);
        Ok(())
    } 
     pub fn add_student(ctx: Context<AddStudent>, student_struct: StudentStruct)-> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        school_account.students.push(student_struct);
        Ok(())
    }
    pub fn suspend_student(ctx: Context<SuspendStudent>, student_address: Pubkey)-> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        let student_index =school_account.students.iter().position(|x| x.address == student_address).unwrap();
        school_account.students[student_index].suspended=true;
        school_account.students[student_index].expelled=false;
        school_account.students[student_index].graduated=false;
        Ok(())
    } 


    // marks student as expelled an removes them from the class
    pub fn rusticate_student(ctx: Context<RusticateStudent>, student_address: Pubkey)-> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        let student_index =school_account.students.iter().position(|x| x.address == student_address).unwrap();
        let student_class_index=school_account.classes.iter().position(|x| x.name==school_account.students[student_index].class).unwrap();
        school_account.classes[student_class_index].students.retain(|&x| x != student_address);
        school_account.students[student_index].suspended=false;
        school_account.students[student_index].expelled=true;
        school_account.students[student_index].graduated=false;
        Ok(())
    }

    //marks students as graduated and removes them from the class
     pub fn graduate_students(ctx: Context<GraduateStudents>, student_address: Vec<Pubkey>)-> Result<()>{
        let school_account= &mut ctx.accounts.school_account;
        for student in &mut student_address.iter(){
            let student_index =school_account.students.iter().position(|x| x.address == *student).unwrap();
            let student_class_index=school_account.classes.iter().position(|x| x.name==school_account.students[student_index].class).unwrap();
            school_account.classes[student_class_index].students.retain(|&x| x != *student);
            school_account.students[student_index].suspended=false;
            school_account.students[student_index].expelled=false;
            school_account.students[student_index].graduated=true;
        }
        Ok(())
    
  } 


  //add a grade to a student 

  pub fn add_grade(ctx: Context<AddGrade>,student_address: Pubkey, teacher_address: Pubkey, subject : String , date: String , score: u32) -> Result<()>{
    let school_account = &mut ctx.accounts.school_account;
    //creating a grade struct 
    let mut new_grade = GradeStruct{
        score: score,
        date: date , 
        teacher: teacher_address,
        // school: school_account,
        approved: false,
        subject: subject ,
    };

    //get student index 
    let student_index = school_account.students.iter().position(|x| x.address == student_address).unwrap();

    school_account.students[student_index].grades.push(new_grade);
    Ok(())
  }

  //add attendance to student

  pub fn add_attendance( ctx: Context<AddAttendance>, student_address: Pubkey, teacher_address: Pubkey, date_time: String, subject:String, present: bool  ) -> Result<()>{
    let school_account = &mut ctx.accounts.school_account;
    
    let mut new_attendance = Attendance{
         subject: subject ,
         date_time: date_time,
         present: present,
         student: student_address,
         teacher: teacher_address,
    };

    let student_index = school_account.students.iter().position(|x| x.address == student_address).unwrap(); 

    school_account.students[student_index].attendance.push(new_attendance);
    
    Ok(())
  }

  //give a student a soft skill
  pub fn add_softskill( ctx: Context<AddSoftSkill>, student_address: Pubkey, teacher_address: Pubkey, date_time: String, skill:String,  ) -> Result<()>{
    let school_account = &mut ctx.accounts.school_account;
    
    let mut new_softskill = SoftSkillStruct{
        skill: skill,
        teacher: teacher_address,
        // school:key,
        approved: false,
        date: date_time,
    };

    let student_index = school_account.students.iter().position(|x| x.address == student_address).unwrap(); 

    school_account.students[student_index].soft_skills.push(new_softskill);
    
    Ok(())
  }
  //



}


//Initialization parameters 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub school_account: Account<'info, SchoolAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct CreateClass<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct PromoteStudent<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct AddTeacher<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct AssignTeacher<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct AssignTimetable<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct AddStudent<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct SuspendStudent<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct RusticateStudent<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}

#[derive(Accounts)]
pub struct GraduateStudents<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}
#[derive(Accounts)]
pub struct ApproveSchemeOfWork<'info>{
  #[account(mut)]
  pub school_account: Account<'info, SchoolAccount>,
}


#[derive(Accounts)]
pub struct AddGrade<'info>{
    #[account(mut)]
    pub school_account: Account<'info, SchoolAccount>
}


#[derive(Accounts)]
pub struct AddAttendance<'info>{
    #[account(mut)]
    pub school_account: Account<'info, SchoolAccount>
}


#[derive(Accounts)]
pub struct AddSoftSkill<'info>{
    #[account(mut)]
    pub school_account: Account<'info, SchoolAccount>
}


/*
DATA STRUCTS
*/
//teacher struct 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TeacherStruct {
    pub name : String, 
    pub address: Pubkey,
    pub classes: Vec<ClassStruct>,
}

//timetable struct
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]

pub struct TimetableStruct{
  pub day: String,
  pub subjects: Vec<String>,
}

//class struct 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ClassStruct{
  pub name: String,
  pub teacher: Pubkey,
  pub students: Vec<Pubkey>,
  pub time_table: Vec<TimetableStruct>,
}

//Grade Struct 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GradeStruct{
    pub score: u32,
    pub date: String, 
    pub teacher: Pubkey,
    // pub school: &mut Pubkey,
    pub approved: bool,
    pub subject: String,
}

//Attendance 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Attendance{
    pub subject: String,
    pub date_time: String,
    pub present: bool,
    pub student: Pubkey,
    pub teacher: Pubkey,
}


//Soft Skill Structure 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SoftSkillStruct{
    pub skill:String,
    pub teacher: Pubkey,
    // pub school: Pubkey,
    pub approved: bool,
    pub date: String,
}

//student 

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StudentStruct{
  pub name: String,
  pub class: String,
  pub address: Pubkey,
  pub suspended: bool,
  pub expelled: bool, 
  pub graduated: bool,
  /* pub parent_address: Pubkey, */
  pub soft_skills: Vec<SoftSkillStruct>,
  pub grades: Vec<GradeStruct>,
  pub attendance : Vec<Attendance>
}

//school account
#[account]
pub struct SchoolAccount {
    pub name: String ,
    pub students : Vec<StudentStruct>,
    pub teachers : Vec<TeacherStruct>,
    pub timetable: Vec<ClassStruct>,
    pub classes: Vec<ClassStruct>
}
