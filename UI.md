# Onyx Horizons Information System UI/UX Specification

This document outlines the user interface and experience design for the Onyx Horizons construction company information system.

## Design Principles

- **Minimalistic**: Clean, uncluttered interfaces with essential elements only
- **Intuitive**: Self-explanatory navigation and workflows
- **Efficient**: Quick access to frequently used functions
- **Responsive**: Works well on different screen sizes
- **Consistent**: Uniform design patterns across all pages

## Color Scheme & Visual Identity

- **Primary Color**: Deep onyx (#0F1621) - representing company name
- **Secondary Color**: Gold/amber (#FFC107) - for highlighting and CTAs
- **Background**: Light gray (#F5F7FA)
- **Text**: Dark gray (#333333) for regular text, black (#000000) for headings
- **Success**: Green (#4CAF50) for positive actions
- **Warning**: Amber (#FFBF00) for caution
- **Error**: Red (#F44336) for errors or destructive actions

## Typography

- **Headings**: Sans-serif font (Inter or similar)
- **Body text**: Sans-serif font (Inter or similar)
- **Font sizes**: 
  - H1: 24px
  - H2: 20px
  - H3: 18px
  - Body: 16px
  - Small text: 14px

## Common UI Components

### Navigation

A left sidebar navigation with collapsible sections for:

1. Dashboard
2. Department Management
3. Site Management
4. Personnel
5. Equipment
6. Clients
7. Brigades
8. Tasks & Materials
9. Reports

Each section expands to show relevant sub-items. A company logo appears at the top of the sidebar.

### Tables

Used for listing entities with the following features:
- Sortable columns (using HTMX to reload sorted data)
- Filterable (using HTMX for dynamic filtering)
- Pagination (using HTMX to load next/previous pages)
- Action buttons for each row (View, Edit, Delete)
- Bulk action checkboxes where appropriate

### Cards

Used to display entity details:
- Clean borders with subtle shadows
- Sectioned content with clear headings
- Edit buttons on each section that transform the section into an editable form

### Forms

- Grouped by logical sections
- Inline validation using HTMX
- Responsive layout (stacks on mobile)
- Clear labeling and placeholder text
- Required fields marked with asterisk

### Buttons

- **Primary**: Gold/amber with dark text for main actions
- **Secondary**: Light gray with dark text for secondary actions
- **Destructive**: Red for delete actions
- **Icon buttons**: For common actions like edit, delete, view

## Page Specifications

### 1. Dashboard

**Content:**
- Quick stats cards showing:
  - Active sites count
  - Total departments and areas
  - Active personnel count
  - Equipment utilization rate
- Recent activity timeline
- Alerts for overdue tasks or material overruns
- Quick access to common queries

**Actions:**
- Links to most-used sections
- Quick search for sites, personnel, or equipment

**Queries Supported:**
- None directly, but provides navigation to all query pages

### 2. Department Management

#### 2.1 Department Listing

**Content:**
- Table of departments with columns:
  - Department Name
  - Supervisor
  - Number of Areas
  - Number of Sites

**Actions:**
- Create New Department button
- Filter by name or supervisor
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #1: Get a list of construction departments and their supervisors

#### 2.2 Department Details

**Content:**
- Department information card (Name, Supervisor)
- Areas tab showing areas in this department
- Equipment tab showing equipment assigned to this department
- Sites tab showing all sites across all areas
- Technical personnel tab showing staff in this department

**Actions:**
- Edit department information
- Add/remove areas
- Assign/unassign equipment
- Assign/unassign supervisors

**Queries Supported:**
- #1: Get list of construction departments and supervisors
- #2: Get list of technical personnel staff of designated department
- #3: Get list of sites being built by the specified department
- #5: Get list of construction equipment assigned to the department

#### 2.3 Area Listing

**Content:**
- Table of areas with columns:
  - Area Name
  - Department
  - Supervisor
  - Number of Sites

**Actions:**
- Create New Area button
- Filter by name, department, or supervisor
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #1: Get a list of construction areas and their supervisors

#### 2.4 Area Details

**Content:**
- Area information card (Name, Department, Supervisor)
- Sites tab showing sites in this area
- Technical personnel tab showing staff in this area

**Actions:**
- Edit area information
- Add/remove sites
- Assign/unassign supervisor

**Queries Supported:**
- #1: Get list of construction areas and supervisors
- #2: Get list of technical personnel staff of designated area
- #3: Get list of sites being built by the specified area

### 3. Site Management

#### 3.1 Site Listing

**Content:**
- Table of sites with columns:
  - Site Name/Description
  - Type (with icon for each type)
  - Area/Department
  - Client
  - Status (Planned/In Progress/Completed)
  - Risk Level

**Actions:**
- Create New Site button
- Advanced filtering by type, area, department, client, status, risk
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #3: Get a list of sites being built by specified department/area

#### 3.2 Site Details

**Content:**
- Site information card with type-specific details
- Construction Schedule tab showing timeline of tasks
- Materials tab showing estimates and actual usage
- Equipment tab showing allocated equipment
- Brigades tab showing assigned brigades
- Reports tab showing completion reports

**Actions:**
- Edit site information
- Manage construction schedule
- Update material estimates/usage
- Allocate/deallocate equipment
- Assign/unassign brigades

**Queries Supported:**
- #4: Get list of workers of brigades working on the site
- #6: Get list of construction equipment allocated to the site
- #7: Get schedule and estimate for the construction
- #8: Get report on the construction of the site
- #10: Get list of construction tasks with exceeded deadlines
- #11: Get list of building materials with estimate excesses

### 4. Personnel Management

#### 4.1 Technical Personnel Listing

**Content:**
- Table of technical staff with columns:
  - Name
  - Qualification (Technician/Technologist/Engineer)
  - Position
  - Department/Area
  - Education Level
  - Is Project Manager

**Actions:**
- Create New Technical Personnel button
- Filter by qualification, position, department
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #2: Get list of technical personnel staff of designated area/department

#### 4.2 Technical Personnel Details

**Content:**
- Personal information card (Name, Photo, Contact)
- Professional details card (Qualification, Position, Education)
- Assignments card (Department/Area supervision)
- Type-specific details section based on qualification

**Actions:**
- Edit personal information
- Edit professional details
- Change assignments
- Upload new photo

**Queries Supported:**
- None directly, but provides detailed view of personnel

#### 4.3 Workers Listing

**Content:**
- Table of workers with columns:
  - Name
  - Profession
  - Brigade (if assigned)
  - Is Brigadier
  - Salary
  - Contact

**Actions:**
- Create New Worker button
- Filter by profession, brigade
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #4: Get list of workers of brigades (when filtered)

#### 4.4 Worker Details

**Content:**
- Personal information card (Name, Photo, Contact)
- Professional details card (Profession, Salary, Union)
- Brigade assignment card
- Profession-specific details section

**Actions:**
- Edit personal information
- Edit professional details
- Change brigade assignment
- Upload new photo

**Queries Supported:**
- None directly, but provides detailed view of worker

### 5. Equipment Management

#### 5.1 Equipment Listing

**Content:**
- Table of equipment with columns:
  - Name
  - Total Amount
  - Available Amount
  - Purchase Date
  - Purchase Cost
  - Fuel Type (if applicable)

**Actions:**
- Create New Equipment button
- Filter by name, availability, fuel type
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #5: Get list of construction equipment (when filtered by department)
- #6: Get list of construction equipment (when filtered by site)

#### 5.2 Equipment Details

**Content:**
- Equipment information card
- Allocation history tab showing where equipment was used
- Current allocation tab showing where equipment is currently allocated

**Actions:**
- Edit equipment information
- Create new allocation
- End existing allocations

**Queries Supported:**
- #5: Get list of construction equipment assigned to department
- #6: Get list of equipment allocated to site or worked during period

### 6. Client Management

#### 6.1 Client Listing

**Content:**
- Table of clients with columns:
  - Name
  - INN
  - Contact Person
  - Is VIP
  - Number of Active Sites

**Actions:**
- Create New Client button
- Filter by name, VIP status
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- None from the specified queries list

#### 6.2 Client Details

**Content:**
- Client information card
- Sites tab showing all sites commissioned by this client

**Actions:**
- Edit client information
- Create new site for this client

**Queries Supported:**
- None from the specified queries list

### 7. Brigade Management

#### 7.1 Brigade Listing

**Content:**
- Table of brigades with columns:
  - ID
  - Brigadier Name
  - Number of Workers
  - Current Site Assignment
  - Previous Site Assignment

**Actions:**
- Create New Brigade button
- Filter by brigadier, site assignment
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #13: Get list of brigades that performed specified construction tasks

#### 7.2 Brigade Details

**Content:**
- Brigade information card
- Workers tab showing assigned workers
- Task history tab showing completed tasks
- Current assignment tab

**Actions:**
- Change brigadier
- Add/remove workers
- Assign to task

**Queries Supported:**
- #12: Get list of construction tasks performed by brigade
- #13: Get list of sites where brigade performed tasks

### 8. Tasks & Materials

#### 8.1 Task Listing

**Content:**
- Table of tasks with columns:
  - Name
  - Site
  - Brigade
  - Start Date
  - Expected End Date
  - Actual End Date
  - Status (Planned, In Progress, Completed, Overdue)

**Actions:**
- Create New Task button
- Filter by site, brigade, status, date range
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #9: Get list of sites where specific task type was performed
- #10: Get list of task types with exceeded deadlines
- #12: Get list of task types performed by brigade

#### 8.2 Task Details

**Content:**
- Task information card
- Materials tab showing estimates and actual usage
- Progress tracking section

**Actions:**
- Edit task information
- Add/edit material estimates
- Update actual material usage
- Mark task as completed

**Queries Supported:**
- #11: Get list of building materials with estimate excesses

#### 8.3 Material Listing

**Content:**
- Table of materials with columns:
  - Name
  - Cost per Unit
  - Units
  - Total Usage Across Sites

**Actions:**
- Create New Material button
- Filter by name, cost range
- Sort by any column
- View/Edit/Delete buttons for each row

**Queries Supported:**
- #11: Get list of building materials with estimate excesses (when filtered)

### 9. Reports & Analytics

#### 9.1 Department Reports

**Content:**
- Department selector
- Date range selector
- Report type selector (Sites, Tasks, Materials, Equipment)
- Generated report area

**Actions:**
- Generate report button
- Export to CSV/PDF
- Print report

**Queries Supported:**
- #3: Get list of sites being built by department
- #5: Get list of equipment assigned to department
- #9, #10, #11: Get lists for department

#### 9.2 Site Reports

**Content:**
- Site selector
- Date range selector
- Report type selector (Schedule, Materials, Equipment, Brigades)
- Generated report area

**Actions:**
- Generate report button
- Export to CSV/PDF
- Print report

**Queries Supported:**
- #4: Get list of workers at site
- #6: Get list of equipment at site
- #7: Get schedule and estimate for site
- #8: Get construction report for site
- #10, #11: Get lists for site

#### 9.3 Brigade Reports

**Content:**
- Brigade selector
- Date range selector
- Generated report showing tasks and sites

**Actions:**
- Generate report button
- Export to CSV/PDF
- Print report

**Queries Supported:**
- #12: Get list of tasks performed by brigade
- #13: Get list of sites where brigade worked

## Workflows

### Creating a New Site

1. User navigates to Site Management > Site Listing
2. Clicks "Create New Site" button
3. Form appears with basic site information fields:
   - Site description
   - Type selection (reveals type-specific fields)
   - Area selection
   - Client selection
   - Risk level
   - Location coordinates
4. After saving basic information, user is directed to detail page
5. User adds construction schedule, materials, assigns brigades

### Assigning Equipment

1. User navigates to Equipment Details page
2. Clicks "Create New Allocation" button
3. Form appears with:
   - Department dropdown
   - Optional site dropdown (updates via HTMX based on department)
   - Amount field
   - Start and end date fields
4. After saving, allocation appears in current allocations tab

### Creating Tasks for a Site

1. User navigates to Site Details page
2. Goes to Construction Schedule tab
3. Clicks "Add Task" button
4. Form appears with:
   - Task name and description
   - Start and expected end dates
   - Brigade assignment dropdown
   - Material requirements section with add/remove buttons
5. After saving, task appears in the schedule

## Responsive Design

The UI adapts to different screen sizes:

- **Desktop**: Full sidebar navigation, multi-column layouts
- **Tablet**: Collapsible sidebar, reduced column layouts
- **Mobile**: Bottom navigation bar, single column layouts, collapsible sections

## HTMX Integration

The system uses HTMX for dynamic updates without JavaScript:

- Tables use `hx-get` for sorting, filtering, and pagination
- Forms use `hx-post` for submission with inline validation feedback
- Dropdowns use `hx-get` to populate dependent fields (e.g., areas based on department)
- Detail pages use `hx-get` to switch between tabs
- Edit buttons use `hx-get` to replace view sections with editable forms
- Save buttons use `hx-post` to submit changes and revert to view mode

## Accessibility Considerations

- Proper heading hierarchy for screen readers
- Sufficient color contrast for all text
- Keyboard navigation support
- ARIA labels for interactive elements
- Form error messages linked to form fields
- Focus indicators for keyboard users

This UI design provides a comprehensive solution for managing all aspects of the Onyx Horizons construction company while satisfying all the specified query requirements in an intuitive and efficient manner.
