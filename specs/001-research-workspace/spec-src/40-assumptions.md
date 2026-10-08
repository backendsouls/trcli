## Assumptions

- **Projects are specified separately**: Organizing records into typed projects (doctoral,
  master's, capstone, independent research, and others) with milestones and tasks is
  specified in `specs/003-research-projects`. Until it is delivered, a workspace behaves as
  a single project.
- **Single researcher, local workspace**: The tool is used by one person at a time on their
  own machine. There are no user accounts, sign-in, or permissions. Staff members are
  records describing people, not accounts that can log in. Sharing a workspace with
  colleagues happens by sharing its files through whatever means the team already uses;
  simultaneous editing by several people is out of scope.
- **Actor identity**: The "actor" in audit entries and manual-step confirmations is the
  person identified by the machine's current user and the workspace's configured researcher
  name; it is not authenticated.
- **Telemetry means measurements of the research work and local tool usage**, kept inside
  the workspace for the researcher's own benefit. It is not usage reporting to the tool's
  authors, and nothing is sent anywhere.
- **Courses are those the researcher takes or teaches** in connection with the research.
  Building course content, managing enrolment, and grading are out of scope.
- **Works offline, looks up online on request**: Every capability except online lookup
  works without a network connection. Lookup uses freely available public catalogues that
  need no account, contacts them only when the researcher asks, and sends only the
  identifier. Searching catalogues by keyword and downloading full texts are out of scope.
- **Results are entered by the researcher**: The tool records the findings the researcher
  names; it does not extract numbers or figures from run outputs by itself, and it does not
  create plots.
- **Research questions are optional but encouraged**: Records can exist without being linked
  to a question; the tool reports unlinked records rather than forbidding them.
- **Datasets and manuscripts are referenced, not stored**: The tool records where they are,
  which version they are, and their fingerprint. Storing, moving, or backing up the data and
  the manuscript text is out of scope.
- **Writing happens elsewhere**: The tool tracks drafts and their bibliography; it is not a
  text editor or a typesetting system.
- **Automated steps are instructions the user's own machine can carry out**. Scheduling work
  on remote clusters or cloud services is out of scope for this specification.
- **Audit entries are kept for the life of the workspace** and are removed only when the
  workspace itself is deleted.
- **Interface language is English**, with dates shown in an unambiguous international form.
- **"etc." in the request** is read as an expectation that further record types can be added
  later in the same shape (create, list, view, update, delete, link, audit). Further record
  types and capabilities (reading annotations, tasks, backup, lab notebook, submissions,
  funding, ethics, and others) are specified separately in `specs/002-research-lifecycle`.
- **Delivery is incremental**: User stories are delivered in priority order, each as its own
  planned and reviewed slice; this specification describes the whole product so that the
  slices stay consistent with one another.
