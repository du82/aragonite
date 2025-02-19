--------------------------------------
## OPEN TECHNOLOGY FUND APPLICATION ##
## DRAFT // BOILERPLATE ##
--------------------------------------

# Several areas where we need to strengthen the application:

>> Problem Statement

We've focused well on technical differentiation, but need more specific examples of information control/censorship
Need to cite specific case studies where existing tools failed during critical periods
Should detail more concrete user problems from our existing community


>> Adversarial Analysis

"What problem will your project address?" - discussing why current solutions are inadequate
"Give a brief overview of activities" - discuss how we'll address these attack vectors
Need to explicitly address how adversaries might try to undermine the system
Should discuss short/medium/long-term adversarial strategies
Need to address potential illicit uses directly


>> Project Planning

Our objectives need more specific, measurable outcomes
Should add clear distribution and promotion plan
Need more detailed milestone metrics


>> Community/Sustainability

Need to better explain our existing community support
Should detail how others can reuse/build on the platform
Need to address long-term sustainability past OTF funding


>> Risk Assessment

Should explicitly discuss risks to implementers and users
Need to address technical limitations in different environments
Should discuss legal challenges more directly


This IS why some privacy tools get funded while others don't - OTF is looking for a very specific combination of technical merit, practical impact, and risk awareness.

+++++++++++++++++++++++++++++++++++++
+++APPLICATION QUESTION VERSIONING+++
+++++++++++++++++++++++++++++++++++++


# Project Title *
## This project name can be changed if a full proposal is requested.

[v1] Aragonite - Privacy Search Engine

# Describe your project in 1-3 sentences.

----

[v1] A truly secure search engine with zero client-side JavaScript and complete Tor compatibility, allowing censorship-resistant access to information through distributed mirrors. The system can operate fully airgapped, online, or offline, with a transparent infrastructure that users can verify. Its minimal attack surface and flexible deployment options make it uniquely suited for privacy-sensitive environments, conflict zones, and regions experiencing internet restrictions.

----

[v2] Aragonite is a platform for creating censorship-resistant, secure, focused information access systems.

----

# What problem will your project address?
## Describe the problem that exists. What change is needed? How will your project contribute to solving this problem?
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#what-problem-will-your-project-address

----

[v1] Current search engines force users to choose between functionality and privacy. 'Private' search engines typically rely heavily on JavaScript, creating numerous security vulnerabilities and making them barely usable over Tor. Meanwhile, users in restricted environments need reliable access to information without exposing themselves to surveillance or tracking. This project solves both problems by providing a fully-functional search engine with virtually no attack surface - it uses zero client-side JavaScript, works seamlessly over Tor, can be easily mirrored to bypass censorship, and can even operate offline or air-gapped. Unlike existing solutions that offer privacy through marketing promises, our system provides privacy through fundamental technical design.

----

[v2] Organizations and individuals in high-risk environments need ways to securely access, archive, and distribute focused sets of information without exposing themselves to surveillance or tracking. Current solutions force a choice between accessibility and security - existing search tools either require trust in centralized providers, depend on complex infrastructure that's difficult to maintain, or create significant security vulnerabilities through their technical design. Privacy-focused alternatives typically rely on JavaScript and complex server architectures, making them unreliable over Tor and leaving numerous attack vectors.

Aragonite solves this by providing a platform for creating and distributing secure, focused information access systems. Its minimal attack surface, ability to create custom indexes, and flexible deployment options (including offline and air-gapped operation) enable organizations to build and maintain their own secure information resources. Unlike existing solutions that offer privacy through policy promises, Aragonite provides security through fundamental technical design, verifiable infrastructure, and the ability to operate even when traditional access methods are blocked.

----

[v3] In regions facing digital repression, access to information is increasingly controlled through sophisticated censorship and surveillance. While privacy-focused search tools exist, they remain vulnerable to blocking, surveillance, and shutdown - particularly during critical moments when information access is most vital. Current solutions force users to trust centralized services that can be compromised or blocked, while self-hosted alternatives require complex infrastructure that's difficult to maintain securely.

This creates severe consequences for journalists, human rights organizations, and communities under surveillance. When information access is restricted during civil unrest or ahead of elections, existing tools often fail precisely when they're most needed. Local organizations cannot maintain their own secure information systems without risking surveillance or blocking. The problem extends beyond search - communities need ways to preserve and access focused collections of critical information even when primary internet access is restricted.

Aragonite addresses this by providing a platform for creating censorship-resistant, secure information access systems. Organizations can build and maintain their own focused indexes of critical information, distribute them through mirrors both on clearnet and Tor, and operate even during internet restrictions. The system's minimal attack surface and verifiable security mean organizations can deploy it without requiring complex infrastructure or exposing themselves to surveillance.

The problem is particularly acute in regions experiencing internet shutdowns or aggressive surveillance, but extends to any context where information access is controlled or monitored. Documented cases of information control during protests and elections show how existing tools often fail when most needed. Without addressing this gap, communities facing repression will continue to lack reliable ways to maintain access to critical information.

Our early community of users has already demonstrated the need for such capabilities, particularly among groups requiring focused access to specific information resources while maintaining strict security requirements. The ability to create and distribute custom indexes through a proven secure platform would provide vital infrastructure for information resistance.

----

# If this project is funded, what form will it take?

() Technological Development

# Give a brief overview of the activities in this project.
## Briefly list key milestones and activities you will carry out.
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#give-a-brief-overview-of-the-activities-in-this-project
----

[v1] The project focuses on refining and scaling our existing functional prototype. Key activities include:

1. Enhancing the indexing system to expand search coverage while maintaining security and privacy guarantees
2. Developing documentation and tools to simplify mirror deployment by trusted third parties
3. Creating educational materials about the system's security model and verification methods
4. Building relationships with organizations that could benefit from deploying mirrors
5. Implementing and testing additional circumvention techniques for accessing mirrors in restricted environments
6. Establishing metrics to measure successful deployment and usage while preserving user privacy

----

[v2] Drawing from our full context, let me lay out a structured plan:
"Objective 1: Strengthen and Scale Core Platform (Months 1-4)

1.1: Enhance indexing system to handle larger datasets while maintaining security guarantees
1.2: Develop and document mirror deployment tools for third parties
1.3: Implement additional circumvention techniques for mirror access
Deliverable: Robust indexing and mirror deployment system

Objective 2: Build General Index with Legal Protection (Months 3-8)

2.1: Develop methodology for ethical index building within legal boundaries
2.2: Create initial general-purpose search index
2.3: Establish mirror distribution network
Deliverable: Working general index with distributed mirror network

Objective 3: Documentation and Educational Materials (Months 6-10)

3.1: Create comprehensive documentation for organizations deploying mirrors
3.2: Develop educational materials about system security model
3.3: Build verification tools and guides for users
Deliverable: Complete documentation and educational package

Objective 4: Open Source Release and Community Building (Months 9-12)

4.1: Prepare codebase for AGPL release
4.2: Create contribution guidelines and security protocols
4.3: Establish support channels for mirror operators
Deliverable: Open source release with community infrastructure

Lead developer Du Cheng will focus on core technical development and security implementation (Objectives 1 and 2). Project lead Garrett Robinson will manage documentation, community building, and strategic planning (Objectives 3 and 4). Both team members will collaborate on index development and mirror network establishment.
The timeline accounts for parallel development while ensuring each phase builds on previous work. The open source release is strategically placed last to ensure proper institutional backing and community infrastructure are in place.

----

# Are there similar projects that exist already? How is your project different or complementary to those projects?
## What gaps does your project address? Does your project build on these projects? If so, how? Is there opportunity for collaboration? Provide web links if available.

----

[v1] While users can access DuckDuckGo, Mojeek, or SearXNG through Tor, this creates fundamental privacy and usability challenges. These services maintain complex server infrastructures that could potentially log or be compromised, requiring users to trust their privacy promises rather than verifiable technical guarantees. They are also highly centralized - if their domains are blocked or servers compromised, users lose access entirely. Even Tor access can be disrupted by ISP-level blocking. Our distributed mirror system provides genuine resilience - even if Tor is blocked and primary domains are censored, users can still access mirrors through various paths. Unlike SearXNG instances which can be poorly maintained or unstable, our simplified architecture enables mirrors to be reliably operated with minimal overhead. Users can verify exactly what code is running through neofetch, rather than trusting privacy policies or hoping volunteer instances are properly configured. Simply accessing existing search engines through Tor provides a false sense of security, while our system provides verifiable privacy guarantees through its fundamental design and demonstrated reliability through its distributed architecture.

----

[v2] While tools like DuckDuckGo, Mojeek, and SearXNG offer privacy-focused search, Aragonite addresses fundamentally different needs. These services maintain complex infrastructures requiring users to trust their privacy promises rather than verifiable technical guarantees. They are also highly centralized - if their domains are blocked or servers compromised, users lose access entirely. Even Tor access can be disrupted by ISP-level blocking.

Aragonite's distributed architecture provides genuine resilience - even if Tor is blocked and primary domains are censored, users can still access mirrors through various paths. Unlike SearXNG instances which can be poorly maintained or unstable, our simplified architecture enables mirrors to be reliably operated with minimal overhead. Users can verify exactly what code is running through neofetch, rather than trusting privacy policies or hoping volunteer instances are properly configured.

Most significantly, while existing tools are single-purpose search engines, Aragonite is a platform for creating secure information access systems. Organizations can build their own focused indexes of content, maintain searchable archives, and distribute these through the same secure infrastructure. This enables use cases far beyond traditional search - from preserving access to critical information during internet shutdowns to maintaining specialized knowledge bases for high-risk environments.

There's potential for collaboration with existing privacy infrastructure projects around mirror distribution networks and Tor integration. Our minimal attack surface and verified deployment model could also provide lessons for other privacy tools struggling with security and reliability challenges. The project builds on proven concepts from existing search privacy tools while fundamentally rethinking how to provide verifiable security and censorship resistance through architectural design rather than policy promises.

----

[v3] Drawing from our complete context and deeper understanding, let me craft a revised response:
"While tools like DuckDuckGo, Mojeek, and SearXNG offer privacy-focused search, Aragonite addresses fundamentally different needs. These services maintain complex infrastructures requiring users to trust their privacy promises rather than verifiable technical guarantees. They are also highly centralized - if their domains are blocked or servers compromised, users lose access entirely. Even Tor access can be disrupted by ISP-level blocking.
Aragonite's distributed architecture provides genuine resilience - even if Tor is blocked and primary domains are censored, users can still access mirrors through various paths. Unlike SearXNG instances which can be poorly maintained or unstable, our simplified architecture enables mirrors to be reliably operated with minimal overhead. Users can verify exactly what code is running through neofetch, rather than trusting privacy policies or hoping volunteer instances are properly configured.

Most significantly, while existing tools are single-purpose search engines, Aragonite is a platform for creating secure information access systems. Organizations can build their own focused indexes of content, maintain searchable archives, and distribute these through the same secure infrastructure. This enables use cases far beyond traditional search - from preserving access to critical information during internet shutdowns to maintaining specialized knowledge bases for high-risk environments.

Our team has demonstrated the ability to create impactful privacy tools through privacyspreadsheet.com, which received 100,000 unique visitors on launch day and has been referenced by privacy resources like nowhere.moe and cited by privacy advocates like Ezra. This experience informs Aragonite's development and shows our ability to reach users who need these tools.

There's potential for collaboration with existing privacy infrastructure projects around mirror distribution networks and Tor integration. The platform's ability to create custom indexes could also complement existing privacy tools by providing secure search capabilities for their specific domains. The project builds on proven concepts from existing search privacy tools while fundamentally rethinking how to provide verifiable security and censorship resistance through architectural design rather than policy promises.

We plan to release the platform under the AGPL license with proper institutional backing, ensuring the technology remains available to the privacy community while preventing commercial exploitation of privacy-focused work.

---

# How long do you estimate this project will take?
## This estimate may be revised if a full proposal is requested.

() 6 months to 1 year

# How much funding do you estimate you will need? (In US Dollars) *
## This estimate can be revised if a full proposal is requested.

[v1] $150,000

# Who would benefit from this project?
## Tell us about the people who will use this project? How is it relevant to them?
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#who-would-benefit-from-this-project

----

[v1] This project primarily benefits users who need reliable, private access to information in challenging environments: journalists and researchers working on sensitive topics, individuals in regions with heavy surveillance or censorship, human rights organizations needing secure search capabilities, and privacy-conscious users who want verifiable rather than promised security. The system's ability to operate offline or through mirrors makes it particularly valuable during internet shutdowns or in conflict zones. Small organizations can easily host verified mirrors, enabling community-driven distribution of access. Privacy-focused libraries and educational institutions can deploy local instances without worrying about complex security configurations. The project's minimal and verifiable architecture also benefits the broader privacy tool ecosystem by demonstrating how to build genuinely secure services without relying on privacy theater. Our existing community of several hundred users has already demonstrated the real-world value for these use cases.

----

[v2] Aragonite serves organizations and individuals who need to maintain secure access to focused information in challenging environments. Human rights organizations can create and distribute verified information archives with genuine privacy guarantees. Journalists and researchers working on sensitive topics can build specialized research databases without compromising security. Communities facing potential internet restrictions can pre-build and mirror, equally between clearnet and Tor, critical information resources before disruptions occur.

The platform particularly benefits groups in regions with heavy surveillance or censorship, who need more than just private search - they need ways to maintain and distribute their own information systems securely. Privacy-focused libraries and educational institutions can deploy local instances with verified security, rather than hoping volunteer instances are properly configured. Small organizations can easily host mirrors of critical information, enabling community-driven distribution of access even when primary channels are blocked.

The project also benefits the broader privacy tool ecosystem by demonstrating how to build genuinely secure services without relying on complex infrastructure or privacy theater. The emphasis on verifiable security and flexible deployment means organizations can trust the platform for critical information needs rather than hoping centralized services remain accessible and trustworthy.

----

[v3] Drawing from our complete context and OTF's specific focus on repressive environments, let me craft a focused response:
"Aragonite serves organizations and individuals who need to maintain secure access to focused information in highly restrictive environments. Three key groups will particularly benefit:

Organizations operating under surveillance and censorship can create and distribute verified information archives without exposing themselves to monitoring. When facing potential internet restrictions or shutdowns, they can pre-build focused indexes of critical information and distribute them through mirrors, equally accessible via clearnet or Tor. This enables continued information access even during critical periods when traditional channels are blocked.
Journalists and researchers working on sensitive topics in repressive environments need ways to maintain secure access to specific information resources. Aragonite enables them to build specialized research databases that can be accessed reliably even when primary sources are blocked or surveillance increases. The platform's minimal attack surface and verifiable security mean they can trust their research activities aren't exposing them to monitoring.
Local privacy-focused groups and community organizations can easily host mirrors of critical information, enabling grassroots distribution of access even when primary channels are blocked. Unlike complex privacy tools that require significant technical expertise to deploy securely, Aragonite's simplified architecture and verification system means small organizations can confidently maintain secure information access for their communities.

Our existing connections with privacy-focused communities have demonstrated these specific needs, particularly around maintaining access to focused information resources during periods of increased restriction. The platform's ability to create custom indexes means each deployment can serve its community's particular information requirements while maintaining consistent security guarantees.
The project also benefits the broader internet freedom ecosystem by demonstrating how to build genuinely secure services without relying on complex infrastructure or privacy theater. The emphasis on verifiable security and flexible deployment means organizations can trust the platform for critical information needs rather than hoping centralized services remain accessible and trustworthy.

----

# Where are your intended users, or audiences located?

() Global

# Why are you, and your team members, the right people to work on this project?
## Describe your relevant expertise, knowledge, your past work.
https://docs.opentech.fund/otf-application-guidebook/appendix-i-concept-note-guide#why-are-you-and-your-team-members-the-right-people-to-work-on-this-project

----

[v1] Our team combines deep technical expertise with proven operational leadership.

Du Cheng brings a decade of programming and IT security experience, having made meaningful contributions to critical privacy projects including Signal, Element, Status, and Safing. His track record includes building an Android project that achieved 500,000 downloads, demonstrating ability to deliver privacy-focused tools at scale.

Garrett Robinson provides strategic and operational leadership, bringing direct experience from both Visa and high-growth startups. His background in managing complex projects and scaling operations complements the technical foundation.

Our team has already demonstrated successful collaboration through privacyspreadsheet.com, which reached 100,000 unique visitors on launch day and has been recognized by respected privacy advocates and platforms including nowhere.moe and Ezra. This project showcased our ability to create privacy tools that resonate with users while maintaining rigorous technical standards.

The combination of deep technical expertise in privacy and security with proven operational leadership makes our team uniquely qualified to scale this search engine while maintaining its core security principles.

----

[v2] Our team combines deep technical expertise with proven operational leadership in the privacy and security space:

Du Cheng brings a decade of programming and IT security experience, with meaningful contributions to critical privacy projects including Signal, Element, Status, and Safing. This work demonstrates deep understanding of building secure, censorship-resistant systems at scale. His track record includes creating an Android project that achieved 500,000 downloads, showing ability to develop and maintain privacy tools that reach significant user bases.

Garrett Robinson provides strategic and operational leadership, bringing experience from both Visa and high-growth startups. This combination of enterprise and startup experience is particularly valuable for managing complex privacy projects that need both security rigor and operational flexibility.

Our team has already demonstrated successful collaboration through privacyspreadsheet.com, which reached 100,000 unique visitors on launch day and has been recognized by respected privacy advocates and platforms including nowhere.moe and Ezra. This project showcased our ability to create privacy tools that resonate with users while maintaining rigorous technical standards.

Most importantly, we've already built a working prototype of Aragonite, proving our ability to execute on complex privacy infrastructure. The platform's minimal attack surface, verifiable security model, and flexible deployment options demonstrate our understanding of real-world privacy tool requirements. Our early community of several hundred users has helped us understand and refine the system based on actual needs in restricted environments.

The combination of proven privacy tool development experience, successful project deployment history, and deep understanding of security architecture makes our team uniquely qualified to scale this platform while maintaining its core security principles.

#### Team_Description_Notes
(((The only thing we might want to expand on slightly would be more detail about experience working with at-risk communities specifically. )))

----

# Please upload any supporting documents to your application.
## Accepted file types are .doc, .docx, .odp, .ods, .odt, .pdf, .ppt, .pptx, .rtf, .txt, .xls, .xlsx

[suggestions:]
>> Technical Architecture Document 

System architecture diagrams/overview
Security model details
Mirror deployment architecture
Index creation process
(A better version of our lil spider drawing)

>> Project Timeline/Roadmap

More detailed breakdown of the activities we outlined
Specific milestones and deliverables
Resource allocation

>> Prototype Documentation

Screenshots/examples of current functionality
Technical specifications
Deployment examples

>> Supporting Data

Stats from privacyspreadsheet.com success
Current user community information
Examples of mirror deployment

>> Team Details

More detailed bios/CVs
Previous project documentation
Relevant technical publications/contributions
[suggestions]

# My application will be dismissed if it does not fit within OTF's mission, values, principles statements. *

() Yes

# I understand that all intellectual property created with support for this application must be openly licensed. *

() Yes

# If my project involves technology development, I agree to undergo a code security audit facilitated by OTF *

() Yes

--------------------------------------

## NOTES ##

+++++++++++++++++++++++++++++++++++
REMEMBER TO INCLUDE LINKS TO THINGS
+++++++++++++++++++++++++++++++++++

1. The OTF has a preference of projects that will meet their goals in 6-12 months.
2. The OTF has a preference for distributing funds between $10,000 - $200,000.

--

"Why even bother"

We do not need any money to build out capability. capability is not necessarily the issue.

1. We require the legal protections to properly build out the index (there is a scaling bottleneck and IP blocking concerns).
2. We require the funds for long-term mirror hosting and domains (along with any other cost, like renewals).
3. We potentially require the funds to bring on another developer.
4. The OTF stamp of approval will be a sign of legitimacy, driving actual adoption and trust.
5. We will not have to worry about someone benefitting/profiting off of our code with a legitimate backer that specializes in this.
6. [potentially] funds can be used for living expenses and other direct cost (operational, travel, if needed).

--

1. The Open Tech Fund's Internet Freedom Fund (IFF) is a crucial source of support, and it's not feasible to sustain Aragonite as a traditional capitalist venture. The nature of the product requires a different approach, one that prioritizes the values of internet freedom and user privacy over profit.

2. Advertisements and user-identifiers are antithetical to Aragonite's value.

3. We're not just looking for funding; we're looking for a partner that shares the vision and is willing to provide meaningful support you in creating an impactful platform in an unfilled niche.

--

1. Application [https://apply.opentech.fund/internet-freedom-fund-concept-note/]
2. Proposal Guide [https://docs.opentech.fund/otf-application-guidebook/appendix-ii-proposal-guide]

--

I believe SimpleX was rejected either due to asking for too much money, or asserting a much longer timeframe than they were comfortable with. Perhaps their operational cost is higher than that of Signal or Briar as well.