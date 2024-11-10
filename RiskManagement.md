# Risk Management 

## Table of Contents

1. [Objective](#objective)
2. [Methods and tools used in the homework](#methods-and-tools-used-in-the-homework)
3. [Project activities](#project-activities)
4. [Risk Management Table](#risk-management-table)
5. [Scope Creep](#scope-creep)
6. [Stakeholder Misalignment](#stakeholder-misalignment)
7. [Non-Compliance With New Regulations](#non-compliance-with-new-regulations)
8. [Incomplete Requirements](#incomplete-requirements)
9. [Inadequate Testing Coverage](#inadequate-testing-coverage)
10. [Sources of Information](#sources-of-information)
11. [Footnotes](#footnotes)

## Objective

To create a Risk Management Matrix to map Project Activity to Risk and assess the risk probability and severity.

### Methods and tools used in the homework

1. The individual homework assignment for the project plan (previous work that has been graded by the teacher);
  1. 1. Kanban boards (used for project and product management teams);
  1. 2. Milestones and issues associated with this repository that hosts the project plan
2. Risk management lecture
3. Using Copilot to cite all the sources of information (and using University of Tartu regulations for citing the information)


##  Project activities

1. Project Planning
2. Requirements gathering (Identify compliance needs)
3. Testing and Review

According to the homework assignment conditions, I have to choose 2 risks associated with these
activities:

1. 1. Scope Creep
1. 2. Stakeholder Misalignment

2. 1. Non-Compliance With New Regulations
2. 2. Incomplete Requirements
  
3. 1. Inadequate Testing Coverage
3. 2. Delay in Testing Due To Resource Constraints
  
To explain these risks, I would like to introduce the following table:

| *Risk Id* | *Risk*                                       | *Description*                        | *Possible solution* |
|-----------|----------------------------------------------|--------------------------------------|---------------------|
| 1         | Scope Creep                                  | Adding new requirements or features without thorough evaluation can lead to scope creep, which may delay project timelines, increase costs, and disrupt deliverables.            | Contribution policies      |
| 2         | Stakeholder Misalignment                     | Miscommunication or lack of alignment between stakeholders can confuse, delays approvals, and incorrect assumptions, ultimately impacting the project's success.             | Conduct regular stakeholder meetings          |
| 3         | Non-Compliance With New Regulations          | Failing to keep up with emerging data privacy regulations could result in legal penalties, and reputational damage, potentially hindering project progress.     | Establish compliance monitoring system         |
| 4         | Incomplete Requirements                      | Rushed or incomplete gathering of compliance requirements may result in missed regulations or critical features, leading to delays and rework. | Conduct thorough requirements workshops          |
| 5         | Inadequate Testing Coverage                  | If testing coverage is too narrow (e.g., under 75%), critical bugs or performance issues could go unnoticed until later stages, risking project success.               | Enhance test coverage with automated testings          |
| 6         | Delay in Testing Due To Resource Constraints | Shortages of skilled testers or testing resources (e.g., tools or personnel) could lead to delays in the testing phase, resulting in missed deadlines and potential quality issues.   | Optimize resource allocation and consider outsourcing          |
> Table 1. Risks associated with project management activities

### Scope Creep 

#### Definition

According to the definition, ***Source Creep**** is a common problem in Project Management that occurs when the project members add new activities (Alana Rudder, 2024, § Definition). 
In other words, they try to add new features that disrupt the work flow and might introduce discrepancies and new bugs, as the project plan adheres
to the strict plan where this "new feature" has not been included yet. 

#### Different Approaches taken by Python and Rust (developers[^1])

The possible solution discusses about how a project team can restrict the access and introduce the contribution guidelines.
What it means is the system should not allow anyone to commit changes, and each change should be reviewed by the assignees[^2]. 
However, most of the time, it is not just changes in the policy or project plan, it might be introducing new features, rules, etc.

##### Python approach

The Python team has a strict approach to releasing their product (Python - the programming languages):

1. Alpha phase: core functionality;
2. Beta phase: after alpha, the Python team avoids any new features. This claim can be found on their official website.
3. Pre-release and experimental versions
4. Official release.

The key element is the fact that they do not add new features to the Beta phase. Not only does it simplify the development
process, but it also helps developers, testers, designers, team leaders, project and product management teams to focus on 
their exact assignment.

##### Rust approach

Rust took another approach - a dynamic one: they allow developers to try the "Nightly" version of the Rust programming language.
It means that despite new features can also be regulated under their policy, their product team is working parallel to the 
project team, meaning that feedback and support are being gathered over time. 

#### Solution

Do not allow immediate action, and always have a back-up system. Restrict the access by roles and introduce responsible people for review.
For the most vital tasks, the company can have a special system: all review people must submit their agreement to their leader and only then
the change is committed. Such a safety measure guarantees that no change is done without leaders knowing. If any change was mistakenly done,
the leader can always back-up, arrange a meeting and discuss their new plan.

### Stakeholder misalignment

#### Definition

According to the definition, "Stakeholder misalignment" occurs when different stakeholders involved in the project have different opinions (Karen Walker & Dorie Clark, 2024, § 1. What to Do When Project Stakeholders Aren’t on the Same Page).

#### Solution

The solution is pretty simple yet very important: to conduct regulated meetings for stakeholders.

### Non-Compliance With New Regulations

#### Definition

The company must need to be in compliance with the legal system all the time, which prevents "Non-Compliance With New Regulations" Risk.
If not handled or mitigated, the risk threats the company with lawsuits and a decrease in reputation (leading to issues with stakeholders) and 
also lowers the revenue, because stakeholders, and potential buyers might not want to buy the product due to the aforementioned problems (Financial Crime Academy, 2024, § Key Takeaways).

#### Solution

Establish a compliance monitoring system or assign this to the people who are going to be responsible for informing the company about the new regulations. 
For this reason, companies hire legal specialists who consult them about these "new regulations" and then outsource data management to the responsible companies.
In other words: 

> To avoid any potential risks (decline in sales, a tarnished brand image, etc), businesses must prioritize compliance efforts and adhere to new regulations, as well as new emerging standards (Financial Crime Academy, 2024, § Reputational Damage).

### Incomplete requirements

Most of the time, a company tries to speed up their product development to satisfy demand on time and achieve a certain quality to match the client's expectancy. However,
during this process, project management might face the classic "incomplete requirements" risk, given by

> Project requirements contribute to the failure of the project itself provided that requirements are poorly gathered or how they are managed by the project manager.
> Projects based on faulty requirements are most likely to fail.
> (John Spacey, 2024, § 13 examples of requirements)

> ***Major reason:*** An inefficient and wrong requirements plan contributes to the failure of the project. 

### Inadequate Testing Coverage

Oops! TODO: Add information

## Sources of Information

1. Scope Creep: Definition, Examples & How To Prevent It Author: Alana Rudder Publisher: Forbes Advisor https://www.forbes.com/advisor/business/scope-creep/: Forbes Advisor Date Accessed: 10 November 2024
2. What to Do When Project Stakeholders Aren’t on the Same Page Authors: Karen Walker and Dorie Clark Publisher: Harvard Business Review https://hbr.org/tip/2023/12/what-to-do-when-project-stakeholders-arent-on-the-same-page: Harvard Business Review Date Accessed: 10 November 2024
3. Understanding The Consequences Of Non-Compliance: Risks And Penalties Publisher: Financial Crime Academy [https://financialcrimeacademy.org](https://financialcrimeacademy.org/consequences-of-non-compliance/#:~:text=Non-compliance%20with%20regulations%20can%20lead%20to%20financial%20penalties%2C,requires%20developing%20policies%2C%20training%20employees%20and%20monitoring%2Fauditing%20processes.): Financial Crime Academy Date Accessed: 10 November 2024
4. 13 Examples of Requirements Risk Author: John Spacey Publisher: Simplicable [https://simplicable.com/new/requirements-risk](https://simplicable.com/new/requirements-risk#:~:text=Incomplete%20Requirements%20Requirements%20that%20are%20incomplete%20leading%20to,that%20make%20no%20mention%20of%20a%20user%20interface.): Simplicable Date Accessed: 10 November 2024
5. Enhancing Software Quality: Effective Test Coverage Techniques Publisher: Katalon https://katalon.com/resources-center/blog/test-coverage-techniques: Katalon Date Accessed: 10 November 2024
6. 

## Footnotes

[^1]: By **developers**, I am referring to the individuals or teams who are responsible for the creation, evolution, and maintenance of the Python and Rust programming languages. These are the people who design the language's features, contribute to its codebase, and determine its release and development cycles.
[^2]: An assignee is an individual whom the task was assigned to (responsible individual for the given task).

