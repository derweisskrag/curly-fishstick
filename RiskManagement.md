# Risk Management 

## Logo

![image](https://github.com/user-attachments/assets/6fa72c3b-8cf8-42e9-ba8e-5e50c9d4e598)


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
10. [Delay in Testing Due To Resource Constraints](#delay-in-testing-due-to-resource-constraints)
11. [Product management risks](#product-management-risks)
12. [Activities](#activities)
13. [Risks](#risks)
14. [Risk management matrix](#risk-management-matrix)
15. [Sources of Information](#sources-of-information)
16. [Footnotes](#footnotes)

## Objective

To create a Risk Management Matrix to map Project Activity to Risk and assess the risk probability and severity.

### Methods and tools used in the homework

1. The individual homework assignment for the project plan (previous work that has been graded by the teacher);
  1. 1. Kanban boards (used for project and product management teams);
  1. 2. Milestones and issues associated with this repository that hosts the project plan
2. Risk management lecture
3. Using Copilot to cite all the sources of information (and using University of Tartu regulations for citing the information)
4. Citing the statistics to back up my quantifiable evidence


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

#### Solution

Conduct thorough requirements workshops that translates to have a comprehensive understanding about what you are trying to achieve.

### Inadequate Testing Coverage

Like any software, a project needs high test coverage (85-100%) according to the ISO/IEC/IEEE 29119-4:2021 protocol. 

> **Disclaimer**: Please, notice that the protocol clearly states that it shall not be liable for any damages or problems
> caused by it. It means that people are responsible for their specific use of this standard (ISO) protocol. Hence, we can
> define our own constant for the required test coverage bound: it is up to responsible people. Generally, this bound occurs
> at 85% or more because the higher test coverage, the higher probability that you included and tested all possible scenarios
> that can happen to your product (misuse, violation, etc).

The testing coverage provides a quantifible way to measure and access the codebase and develop new testing strategies for testing the code (Katalon, 2024, § Importance of Test Coverage in Software Quality Assurance). This helps to understand which part of the software or project are thorougly tested. 

What contributes to the low test coverage? The modern software development is complex process involving several key details (Katalon, 2024, § Challenges in Achieving Comprehensive Test Coverage):

1. Complexity of Modern Software Systems: complex architecture of a project leads to the increased difficulty of testing each part;
2. Rapid Development Cycles: Speed & Quality - it is not always possible to satisfy all the demand;
3. Resource Constraints: Requires much of the time & excellent testers[^3] whose testing toolkit is always at their disposal, ready to be used;
4. Evolving Requirements: constant vigilance & adaptability from the QA team.

Therefore, it is important to handle testing carefully and prevent any inadequate testing risks.

#### Solution

1. Hire a good testing team that includes excellent testers.
2. Automate the testing system and ensure it checks for the coverage
3. Follow the ISO/IEC/IEEE 29119-4:2021 testing model to ensure all deliverables. 

For the second solution, I would like to point out that testing with profile allows testers to see how much of the code is actually tested.

### Delay in Testing Due To Resource Constraints

According to available information, the hardest challenge is to achieve the highest test coverage given the lack of resources allocated for the project (Katalon, 2024, § Advanced Test Coverage Techniquese). Not only is it arduous to achieve the goal of the project and ensure all tests are passed due to resources, but advanced techniques require more time to do so. Hence, project needs more time allocated for the testing part.

#### My Kanban board:

![image](https://github.com/user-attachments/assets/c37bedf3-df0b-4e02-b6aa-e76dceb2546f)

So, I allocated 3 bussiness days which can include:

1. Minimum: 8 hours per day, so `3 * 8 = 24` hours for testing
2. Maximum: 12 hours per day, so `3 * 12 = 36` hours for testing

Lack of people, and time, usually leads to overworking and we can confidently say that the time is somewhere in `24 < t < 36`. According to the teacher feedback, and my Kanban board for the product management team, testing can be done in parallel. The QA teams work constantly and gather all the feedback due to how good some companies manage to automate the testing model. This, however, requires good experise to do so.

#### Solution

Optimize the resources allocated to the project. Thus, we have to find and use the techniques to optimize the resources. 
Taking the ISO standard into consideration, we know that it is up to the companies to decide if they follow or not the protocol. 
It clearly implies that the testing and resource allocation models are created by the companies. 

> They can potentially outsource the resource management to the third company to ensure proper resource allocation given the lack of
> expertise, as it can lead to `Delay in Testing Due To Resource Constraints`.

## Product management risks

### Activities

From the product activities, I chose:

1. System Analysis
2. Support & Feedback

### Risks

| *Risk Id* | *Risk*                                       | *Description*                        | *Possible solution* |
|-----------|----------------------------------------------|--------------------------------------|---------------------|
| 1         | Data Integration Challenges | Legacy issues or complex data structures could cause difficulties in system integration. | S1 |
| 2         | Incomplete Data Access | Lack of access to complete data may cause inaccurate analysis and inefficient system design. | S2 |
| 3         | Inadequate Feedback Collection | Ineffective feedback mechanisms may lead to missing valuable user insights. | S3 |
| 4         | Delayed Feedback | Delayed feedback collection can hinder timely resolution of critical issues and user dissatisfaction. | S4 |
> Table 2. Risks associated with product management activities

#### S1: Data standardization

Adopt data standardization and transformation tools: Use ETL (Extract, Transform, Load) tools to standardize data formats, and establish clear protocols for data integration with legacy systems. Consider using middleware to facilitate integration across platforms.

#### S2: Data access protocols & permissions 

Establish data access protocols and permissions: Define and enforce access requirements with stakeholders and set up secure data access agreements. Regularly review data permissions to ensure access completeness and compliance.

#### S3: Divesrse feedback channels

Implement diverse feedback channels: Use multiple channels, such as surveys, focus groups, and user interviews, to capture comprehensive feedback. Consider in-app feedback tools to collect real-time insights directly from users.

#### S4: Automation of feedback collection

Automate feedback collection and prioritize response processes: Set up automated collection for frequent feedback intervals and establish a priority system for addressing urgent issues. Use tools like sentiment analysis to identify critical feedback that needs immediate attention.

## Risk management matrix

### Activity & Risks table

| Risk ID | Activity                                      | Risk                                    | Description                                                                                   |
|---------|----------------------------------------------|-----------------------------------------|-----------------------------------------------------------------------------------------------|
| 1       | Project Planning - Define Project Scope      | Scope Creep                             | New requirements could be added, causing delays and increased costs.                          |
| 2       | Project Planning - Define Project Scope      | Stakeholder Misalignment                | Misunderstandings between stakeholders could delay approval and cause confusion.              |
| 3       | Requirements Gathering - Identify Compliance Needs | Non-Compliance with New Regulations | Overlooking emerging regulations could result in legal and reputational risks.                |
| 4       | Requirements Gathering - Identify Compliance Needs | Incomplete Requirements               | Rushed gathering of compliance needs may lead to errors or delays.                            |
| 5       | Testing and Review - Develop Testing Strategy | Inadequate Testing Coverage            | Critical bugs may go undetected if testing strategy isn’t comprehensive.                      |
| 6       | Testing and Review - Develop Testing Strategy | Delay in Testing Due to Resource Constraints | Lack of skilled testers or resources could delay the testing phase.                  |
| 7       | System Analysis - Analyze the Existing Data Management | Data Integration Challenges          | Data integration may be difficult due to legacy system issues.                                |
| 8       | System Analysis - Analyze the Existing Data Management | Incomplete Data Access               | Lack of access to all data could result in inaccurate analysis.                               |
| 9       | Support and Feedback - Gather User Feedback Post-Implementation | Inadequate Feedback Collection     | Feedback mechanisms may fail, leading to missed insights.                                     |
| 10      | Support and Feedback - Gather User Feedback Post-Implementation | Delayed Feedback                    | Delayed feedback collection could hinder resolution of critical issues.                       |
> Table 3. The Activity & Risk tables

### Risks evaluation table

According to the statistics (TeamStage, 2024, Project Management statistics), I evaluate the risks as follows:

| Risk ID | Probability | Impact | Risk Score |
|---------|-------------|--------|------------|
| 1       | High        | High   | 9          |
| 2       | Medium      | High   | 6          |
| 3       | Medium      | High   | 6          |
| 4       | Medium      | Medium | 4          |
| 5       | High        | High   | 9          |
| 6       | Medium      | High   | 6          |
| 7       | Medium      | Medium | 4          |
| 8       | High        | Medium | 6          |
| 9       | Medium      | Medium | 4          |
| 10      | Low         | High   | 3          |
> Table 4. The risk evaluation

Where

| Probability Measure | Range   |
|---------------------|---------|
| Low                 | 0-30%   |
| Medium              | 30-50%  |
| High                | 60-100% |
> Table 5. How to measure probability

| Impact measure | Range   | 
|----------------|---------|
| Low            | 1-3     |
| Medium         | 4-6     |
| High           | 7-9     |
> Table 6. How to measure impact

The Risk score is based on the Impact which is quantifible way to measure the risk.

### Reasoning and Explanation

#### Project Planning

As shown by statisitcs, the Project Planning is very important and common risks such as ```Scope Creep``` and ```Stakeholder misalignment``` are quite often, and more likely to occur. The also define if the project is going to be successful or miserably fail:

> "Undoubtedly, robust project management improves the chances for success, drives cost-saving, and risk reduction. The Pulse project management statistics show > 
> that high-performing organizations with proven PM practices in place have met their original goals 2.5 times more often (89% vs. 34%)." by (TeamStage, 2024,
> Project Management statistics, § General Project Management Stats & Facts, 5. Projects are 2.5 times more successful when PM practices are implemented.)

I also found out 

> "Stakeholder engagement is the most valuable PM process." by (TeamStage, 2024,
> Project Management statistics, § Top Project Management Statistics: Editor’s Choice)

Meaning that the risk is definitely not low.

#### Gathering Requirements

The data from the statistics agrees with my "Medium" rank for this risk. Indeed, if you know what to do, and have good tools and techniques (such as parametrized testing, automated QA support, etc), your project is more likely to be successful.

> "Setting clear goals helps you to track the milestones and the progress, giving you a clear picture of where you are at the moment. Thanks to this data you can > make some tweaks or reinforce the practices that benefit your end goal." by (TeamStage, 2024, Project Management statistics, § Project Failure Statistics, 1. A > lack of clear goals is the most common factor (37%) for project failure.)

#### Testing and Review

From my table, we know that test coverage is common risk, and most likely to occur, as well as very important because if company deliver bad product to the market,
it immediately affects the stakeholders and clients, thereby contributing to the decrease in reputation for the company (as shown above in the evidence).

> "The latest project management stats show that 58% of organizations fully understand the value of implementing project management as a way to achieve better performance. This means that 42% of companies undervalue the importance of project management as a crucial component for project success." by (TeamStage, 2024, Project Management Statistics, § General Project Management Stats & Facts, 4. 42% of companies don’t understand the need or importance of project management.)

Found from statistics, it turns out that the proper use of resources, and thus, the risk "delay in testing due to resources" is quite often. The "60%" contributes to "High" rank of the risk: it is very often and one of the most hardest.

> "60% of respondents point to poor resource management as their biggest challenge. Other issues included poorly trained project sponsors (33%) ineffective PPM solution deployment (30%), and lack of governance (26%)." by (TeamStage, 2024, Project Management Statistics, § Project Management Performance Statistics, 26. Poor resource management was the biggest PM challenge in 2019.)

These findings agree with my scores: "High" for "low test coverage" and "Medium" for delay in test due to resources, but the risk "poor resource management" is very high according to the statistics.

#### System Analysis - Analyze the Existing Data Management

So, the gathering all the data and testing it thoroughly, as well as to properly manage (proper management) is all important. Thus, making the risk "Incomplete data" and "data integration" very difficult risks to handle. It means that for the QWERTY123, we also need that proper management and make educative and informed decisions, rather relying on low amount of data and fail due to improper management.

> "The attempt of Coca-Cola to introduce New Coke in 1985 is one of the most popular poor project management examples. Instead of conducting in-depth marketing research, the team administered more than 200,000 taste tests to confirm that the subjects liked the taste of New Coke more than the classic one and Pepsi. As it turned out, consumers hated it and Coca-Cola learned a very important lesson — every project needs proper management." by (TeamStage, 2024, Project Management Statistics, § Project Failure Statistics, 13. Coca-Cola wasted $4 million in development and another $30 million in back stocked products after releasing New Coke.)

The following agrees with the fact that project management needs more information (data) to make good decisions to avoid improper management:

> "IT project failure statistics show that 75% of respondents think their projects are always or usually doomed to fail from the start. Out of this 75%, 27% constantly feel this way. At the same time, the majority of respondents (80%) have admitted to spending half their time reshaping the projects." by (TeamStage, 2024, Project Management Statistics, § Project Failure Statistics, 8. 75% of respondents in the IT industry lack confidence in project success.)


#### Support and Feedback - Gather User Feedback Post-Implementation

The customer satisfaction and feedback are very important.

> "While business value was cited as the second most popular measure (46%) of an agile initiative’s success, it ranked as the 
eleventh most popular measure (23%) of an agile project’s success. Velocity (67%) continues to be the number one measure of 
an agile project’s success" by (VersionOne, 2024, 11th Annual State of Agile, § How Success Is Measured... with Agile Projects)

28% of respondents, according to the data referenced in the "11th Annul State of Agile", think that customer and user satisfaction is very important
factor in their bussiness. Thus, the risk "Inadequate Feedback Collection" or "Delayed feedback" turn out to be significant for QWERTY123 company, because
we have proper management for QA and feedback, or we might fail: 28% says it all - not critical, but very imporant.

## Sources of Information

1. Scope Creep: Definition, Examples & How To Prevent It Author: Alana Rudder Publisher: Forbes Advisor https://www.forbes.com/advisor/business/scope-creep/: Forbes Advisor Date Accessed: 10 November 2024
2. What to Do When Project Stakeholders Aren’t on the Same Page Authors: Karen Walker and Dorie Clark Publisher: Harvard Business Review https://hbr.org/tip/2023/12/what-to-do-when-project-stakeholders-arent-on-the-same-page: Harvard Business Review Date Accessed: 10 November 2024
3. Understanding The Consequences Of Non-Compliance: Risks And Penalties Publisher: Financial Crime Academy [https://financialcrimeacademy.org](https://financialcrimeacademy.org/consequences-of-non-compliance/#:~:text=Non-compliance%20with%20regulations%20can%20lead%20to%20financial%20penalties%2C,requires%20developing%20policies%2C%20training%20employees%20and%20monitoring%2Fauditing%20processes.): Financial Crime Academy Date Accessed: 10 November 2024
4. 13 Examples of Requirements Risk Author: John Spacey Publisher: Simplicable [https://simplicable.com/new/requirements-risk](https://simplicable.com/new/requirements-risk#:~:text=Incomplete%20Requirements%20Requirements%20that%20are%20incomplete%20leading%20to,that%20make%20no%20mention%20of%20a%20user%20interface.): Simplicable Date Accessed: 10 November 2024
5. Enhancing Software Quality: Effective Test Coverage Techniques Publisher: Katalon https://katalon.com/resources-center/blog/test-coverage-techniques: Katalon Date Accessed: 10 November 2024
6. Project Management Statistics 2024: New Trends Publisher: TeamStage https://teamstage.io/project-management-statistics/: TeamStage Date Accessed: 10 November 2024
7. Software Testing Effort Estimation and Related Problems: A Systematic Literature Review Authors: Ilona Bluemke and Agnieszka Malanowska Publisher: ACM Digital Library https://dl.acm.org/doi/fullHtml/10.1145/3442694: ACM Digital Library Date Accessed: 10 November 2024
8. VersionOne 11th Annual State of Agile Report Publisher: VersionOne https://www.agile247.pl/wp-content/uploads/2017/04/versionone-11th-annual-state-of-agile-report.pdf: VersionOne Date Accessed: 10 November 2024


## Footnotes

[^1]: By **developers**, I am referring to the individuals or teams who are responsible for the creation, evolution, and maintenance of the Python and Rust programming languages. These are the people who design the language's features, contribute to its codebase, and determine its release and development cycles.
[^2]: An assignee is an individual whom the task was assigned to (responsible individual for the given task).
[^3]: An excellent test is meant to be an individual who has gained high clearance and excellent comprehensive understanding about the testing field, as well as knowledge of the best techniques and tools to be used for specific part of the project to be tested with.
