# Onyx Horizons Backend API Endpoints

This document describes all HTTP endpoints required to implement the Onyx Horizons information system. The endpoints are divided into two categories:

1. **Page Endpoints**: Return full HTML pages
2. **HTMX Endpoints**: Return HTML fragments for dynamic updates

## Common Query Parameters

Many endpoints support these common query parameters:

- `page`: Page number for pagination (default: 1)
- `per_page`: Number of items per page (default: 20)
- `sort_by`: Column name to sort by
- `sort_dir`: Sort direction (`asc` or `desc`)
- `q`: Search query string

## 1. General Endpoints

### Page Endpoints

| Method | Path          | Description                 | Response                   |
|--------|---------------|-----------------------------|----------------------------|
| GET    | `/`           | Dashboard page              | HTML - Dashboard page      |
| GET    | `/404`        | Not found page              | HTML - 404 page            |
| GET    | `/500`        | Server error page           | HTML - 500 page            |

### HTMX Endpoints

| Method | Path                     | Description                               | Response                          |
|--------|--------------------------|-------------------------------------------|-----------------------------------|
| GET    | `/components/alerts`     | Fetch dynamic alerts for dashboard        | HTML fragment - Alert components  |
| GET    | `/components/stats`      | Fetch quick stats for dashboard           | HTML fragment - Stats components  |
| GET    | `/components/activity`   | Fetch recent activity timeline            | HTML fragment - Activity timeline |
| GET    | `/search`                | Global search function                    | HTML fragment - Search results    |

## 2. Department Management Endpoints

### Page Endpoints

| Method | Path                           | Description                    | Response                        |
|--------|--------------------------------|--------------------------------|---------------------------------|
| GET    | `/departments`                 | Department listing page        | HTML - Department listing page  |
| GET    | `/departments/new`             | New department form page       | HTML - Department form page     |
| GET    | `/departments/{id}`            | Department details page        | HTML - Department details page  |
| GET    | `/departments/{id}/edit`       | Edit department page           | HTML - Department edit page     |
| GET    | `/areas`                       | Area listing page              | HTML - Area listing page        |
| GET    | `/areas/new`                   | New area form page             | HTML - Area form page           |
| GET    | `/areas/{id}`                  | Area details page              | HTML - Area details page        |
| GET    | `/areas/{id}/edit`             | Edit area page                 | HTML - Area edit page           |

### HTMX Endpoints

| Method | Path                                     | Description                             | Response                               |
|--------|------------------------------------------|-----------------------------------------|----------------------------------------|
| GET    | `/api/departments`                       | Fetch departments with filters          | HTML fragment - Department table rows  |
| POST   | `/api/departments`                       | Create new department                   | HTML fragment - Success notification   |
| GET    | `/api/departments/{id}`                  | Fetch department details                | HTML fragment - Department details     |
| PUT    | `/api/departments/{id}`                  | Update department                       | HTML fragment - Updated details        |
| DELETE | `/api/departments/{id}`                  | Delete department                       | HTML fragment - Success notification   |
| GET    | `/api/departments/{id}/areas`            | Fetch areas for department              | HTML fragment - Areas table rows       |
| GET    | `/api/departments/{id}/equipment`        | Fetch equipment for department          | HTML fragment - Equipment table rows   |
| GET    | `/api/departments/{id}/sites`            | Fetch sites for department              | HTML fragment - Sites table rows       |
| GET    | `/api/departments/{id}/personnel`        | Fetch personnel for department          | HTML fragment - Personnel table rows   |
| GET    | `/api/areas`                             | Fetch areas with filters                | HTML fragment - Area table rows        |
| POST   | `/api/areas`                             | Create new area                         | HTML fragment - Success notification   |
| GET    | `/api/areas/{id}`                        | Fetch area details                      | HTML fragment - Area details           |
| PUT    | `/api/areas/{id}`                        | Update area                             | HTML fragment - Updated details        |
| DELETE | `/api/areas/{id}`                        | Delete area                             | HTML fragment - Success notification   |
| GET    | `/api/areas/{id}/sites`                  | Fetch sites for area                    | HTML fragment - Sites table rows       |
| GET    | `/api/areas/{id}/personnel`              | Fetch personnel for area                | HTML fragment - Personnel table rows   |
| GET    | `/api/departments/supervisors`           | Fetch available supervisors for dept.   | HTML fragment - Supervisor selector    |
| GET    | `/api/areas/supervisors`                 | Fetch available supervisors for area    | HTML fragment - Supervisor selector    |

## 3. Site Management Endpoints

### Page Endpoints

| Method | Path                           | Description                    | Response                        |
|--------|--------------------------------|--------------------------------|---------------------------------|
| GET    | `/sites`                       | Site listing page              | HTML - Site listing page        |
| GET    | `/sites/new`                   | New site form page             | HTML - Site form page           |
| GET    | `/sites/{id}`                  | Site details page              | HTML - Site details page        |
| GET    | `/sites/{id}/edit`             | Edit site page                 | HTML - Site edit page           |

### HTMX Endpoints

| Method | Path                                     | Description                             | Response                                 |
|--------|------------------------------------------|-----------------------------------------|------------------------------------------|
| GET    | `/api/sites`                             | Fetch sites with filters                | HTML fragment - Site table rows          |
| POST   | `/api/sites`                             | Create new site                         | HTML fragment - Success notification     |
| GET    | `/api/sites/{id}`                        | Fetch site details                      | HTML fragment - Site details             |
| PUT    | `/api/sites/{id}`                        | Update site                             | HTML fragment - Updated details          |
| DELETE | `/api/sites/{id}`                        | Delete site                             | HTML fragment - Success notification     |
| GET    | `/api/sites/{id}/schedule`               | Fetch construction schedule             | HTML fragment - Schedule table           |
| GET    | `/api/sites/{id}/materials`              | Fetch materials usage                   | HTML fragment - Materials table          |
| GET    | `/api/sites/{id}/equipment`              | Fetch allocated equipment               | HTML fragment - Equipment table          |
| GET    | `/api/sites/{id}/brigades`               | Fetch assigned brigades                 | HTML fragment - Brigades table           |
| GET    | `/api/sites/{id}/reports`                | Fetch site reports                      | HTML fragment - Reports table            |
| GET    | `/api/sites/type-fields`                 | Fetch form fields for selected site type| HTML fragment - Type-specific form fields|
| POST   | `/api/sites/{id}/tasks`                  | Create new task for site                | HTML fragment - Success notification     |
| GET    | `/api/sites/by-department/{dept_id}`     | Fetch sites by department               | HTML fragment - Site table rows          |
| GET    | `/api/sites/by-area/{area_id}`           | Fetch sites by area                     | HTML fragment - Site table rows          |
| GET    | `/api/sites/risk-levels`                 | Fetch available risk levels             | HTML fragment - Risk level selector      |

## 4. Personnel Management Endpoints

### Page Endpoints

| Method | Path                                  | Description                          | Response                              |
|--------|---------------------------------------|--------------------------------------|---------------------------------------|
| GET    | `/personnel/technical`                | Technical personnel listing page     | HTML - Technical personnel page       |
| GET    | `/personnel/technical/new`            | New technical personnel form         | HTML - Technical personnel form       |
| GET    | `/personnel/technical/{id}`           | Technical personnel details page     | HTML - Technical personnel details    |
| GET    | `/personnel/technical/{id}/edit`      | Edit technical personnel page        | HTML - Technical personnel edit form  |
| GET    | `/personnel/workers`                  | Workers listing page                 | HTML - Workers page                   |
| GET    | `/personnel/workers/new`              | New worker form                      | HTML - Worker form                    |
| GET    | `/personnel/workers/{id}`             | Worker details page                  | HTML - Worker details                 |
| GET    | `/personnel/workers/{id}/edit`        | Edit worker page                     | HTML - Worker edit form               |

### HTMX Endpoints

| Method | Path                                             | Description                             | Response                                     |
|--------|--------------------------------------------------|-----------------------------------------|----------------------------------------------|
| GET    | `/api/personnel/technical`                       | Fetch technical personnel with filters  | HTML fragment - Technical personnel rows     |
| POST   | `/api/personnel/technical`                       | Create new technical personnel          | HTML fragment - Success notification         |
| GET    | `/api/personnel/technical/{id}`                  | Fetch technical personnel details       | HTML fragment - Technical personnel details  |
| PUT    | `/api/personnel/technical/{id}`                  | Update technical personnel              | HTML fragment - Updated details              |
| DELETE | `/api/personnel/technical/{id}`                  | Delete technical personnel              | HTML fragment - Success notification         |
| GET    | `/api/personnel/technical/qualifications`        | Fetch available qualifications          | HTML fragment - Qualification selector       |
| GET    | `/api/personnel/technical/positions`             | Fetch available positions               | HTML fragment - Position selector            |
| GET    | `/api/personnel/workers`                         | Fetch workers with filters              | HTML fragment - Worker table rows            |
| POST   | `/api/personnel/workers`                         | Create new worker                       | HTML fragment - Success notification         |
| GET    | `/api/personnel/workers/{id}`                    | Fetch worker details                    | HTML fragment - Worker details               |
| PUT    | `/api/personnel/workers/{id}`                    | Update worker                           | HTML fragment - Updated details              |
| DELETE | `/api/personnel/workers/{id}`                    | Delete worker                           | HTML fragment - Success notification         |
| GET    | `/api/personnel/workers/professions`             | Fetch available professions             | HTML fragment - Profession selector          |
| GET    | `/api/personnel/workers/type-fields/{profession}`| Fetch form fields for selected prof.    | HTML fragment - Profession-specific fields   |
| GET    | `/api/personnel/by-department/{dept_id}`         | Fetch personnel by department           | HTML fragment - Personnel table rows         |
| GET    | `/api/personnel/by-area/{area_id}`               | Fetch personnel by area                 | HTML fragment - Personnel table rows         |
| GET    | `/api/personnel/by-brigade/{brigade_id}`         | Fetch workers by brigade                | HTML fragment - Worker table rows            |

## 5. Equipment Management Endpoints

### Page Endpoints

| Method | Path                                | Description                 | Response                    |
|--------|-------------------------------------|-----------------------------|-----------------------------|
| GET    | `/equipment`                        | Equipment listing page      | HTML - Equipment page       |
| GET    | `/equipment/new`                    | New equipment form          | HTML - Equipment form       |
| GET    | `/equipment/{id}`                   | Equipment details page      | HTML - Equipment details    |
| GET    | `/equipment/{id}/edit`              | Edit equipment page         | HTML - Equipment edit form  |

### HTMX Endpoints

| Method | Path                                            | Description                              | Response                                  |
|---------|------------------------------------------------|------------------------------------------|-------------------------------------------|
| GET    | `/api/equipment`                                | Fetch equipment with filters             | HTML fragment - Equipment table rows      |
| POST   | `/api/equipment`                                | Create new equipment                     | HTML fragment - Success notification      |
| GET    | `/api/equipment/{id}`                           | Fetch equipment details                  | HTML fragment - Equipment details         |
| PUT    | `/api/equipment/{id}`                           | Update equipment                         | HTML fragment - Updated details           |
| DELETE | `/api/equipment/{id}`                           | Delete equipment                         | HTML fragment - Success notification      |
| GET    | `/api/equipment/{id}/allocations`               | Fetch equipment allocations              | HTML fragment - Allocations table         |
| GET    | `/api/equipment/{id}/allocation-history`        | Fetch allocation history                 | HTML fragment - Allocation history table  |
| POST   | `/api/equipment/{id}/allocations`               | Create new equipment allocation          | HTML fragment - Success notification      |
| PUT    | `/api/equipment/allocations/{allocation_id}`    | Update equipment allocation              | HTML fragment - Updated allocation        |
| DELETE | `/api/equipment/allocations/{allocation_id}`    | Delete equipment allocation              | HTML fragment - Success notification      |
| GET    | `/api/equipment/by-department/{dept_id}`        | Fetch equipment by department            | HTML fragment - Equipment table rows      |
| GET    | `/api/equipment/by-site/{site_id}`              | Fetch equipment by site                  | HTML fragment - Equipment table rows      |
| GET    | `/api/equipment/fuel-types`                     | Fetch available fuel types               | HTML fragment - Fuel type selector        |

## 6. Client Management Endpoints

### Page Endpoints

| Method | Path                        | Description                 | Response                   |
|--------|-----------------------------|-----------------------------|----------------------------|
| GET    | `/clients`                  | Client listing page         | HTML - Client page         |
| GET    | `/clients/new`              | New client form             | HTML - Client form         |
| GET    | `/clients/{id}`             | Client details page         | HTML - Client details      |
| GET    | `/clients/{id}/edit`        | Edit client page            | HTML - Client edit form    |

### HTMX Endpoints

| Method | Path                                     | Description                             | Response                                 |
|--------|------------------------------------------|-----------------------------------------|------------------------------------------|
| GET    | `/api/clients`                           | Fetch clients with filters              | HTML fragment - Client table rows        |
| POST   | `/api/clients`                           | Create new client                       | HTML fragment - Success notification     |
| GET    | `/api/clients/{id}`                      | Fetch client details                    | HTML fragment - Client details           |
| PUT    | `/api/clients/{id}`                      | Update client                           | HTML fragment - Updated details          |
| DELETE | `/api/clients/{id}`                      | Delete client                           | HTML fragment - Success notification     |
| GET    | `/api/clients/{id}/sites`                | Fetch sites for client                  | HTML fragment - Sites table rows         |
| GET    | `/api/clients/check-inn/{inn}`           | Check if INN is unique                  | HTML fragment - Validation message       |

## 7. Brigade Management Endpoints

### Page Endpoints

| Method | Path                          | Description                 | Response                     |
|--------|-------------------------------|-----------------------------|------------------------------|
| GET    | `/brigades`                   | Brigade listing page        | HTML - Brigade page          |
| GET    | `/brigades/new`               | New brigade form            | HTML - Brigade form          |
| GET    | `/brigades/{id}`              | Brigade details page        | HTML - Brigade details       |
| GET    | `/brigades/{id}/edit`         | Edit brigade page           | HTML - Brigade edit form     |

### HTMX Endpoints

| Method | Path                                     | Description                             | Response                                 |
|--------|------------------------------------------|-----------------------------------------|------------------------------------------|
| GET    | `/api/brigades`                          | Fetch brigades with filters             | HTML fragment - Brigade table rows       |
| POST   | `/api/brigades`                          | Create new brigade                      | HTML fragment - Success notification     |
| GET    | `/api/brigades/{id}`                     | Fetch brigade details                   | HTML fragment - Brigade details          |
| PUT    | `/api/brigades/{id}`                     | Update brigade                          | HTML fragment - Updated details          |
| DELETE | `/api/brigades/{id}`                     | Delete brigade                          | HTML fragment - Success notification     |
| GET    | `/api/brigades/{id}/workers`             | Fetch workers in brigade                | HTML fragment - Workers table rows       |
| POST   | `/api/brigades/{id}/workers`             | Add worker to brigade                   | HTML fragment - Success notification     |
| DELETE | `/api/brigades/{id}/workers/{worker_id}` | Remove worker from brigade              | HTML fragment - Success notification     |
| GET    | `/api/brigades/{id}/tasks`               | Fetch tasks performed by brigade        | HTML fragment - Tasks table rows         |
| GET    | `/api/brigades/{id}/sites`               | Fetch sites where brigade worked        | HTML fragment - Sites table rows         |
| GET    | `/api/brigades/brigadiers`               | Fetch available brigadiers              | HTML fragment - Brigadier selector       |
| GET    | `/api/brigades/by-task/{task_id}`        | Fetch brigades by task                  | HTML fragment - Brigade table rows       |
| GET    | `/api/brigades/by-site/{site_id}`        | Fetch brigades by site                  | HTML fragment - Brigade table rows       |

## 8. Tasks & Materials Endpoints

### Page Endpoints

| Method | Path                           | Description                  | Response                         |
|--------|--------------------------------|------------------------------|----------------------------------|
| GET    | `/tasks`                       | Task listing page            | HTML - Task page                 |
| GET    | `/tasks/new`                   | New task form                | HTML - Task form                 |
| GET    | `/tasks/{id}`                  | Task details page            | HTML - Task details              |
| GET    | `/tasks/{id}/edit`             | Edit task page               | HTML - Task edit form            |
| GET    | `/materials`                   | Material listing page        | HTML - Material page             |
| GET    | `/materials/new`               | New material form            | HTML - Material form             |
| GET    | `/materials/{id}`              | Material details page        | HTML - Material details          |
| GET    | `/materials/{id}/edit`         | Edit material page           | HTML - Material edit form        |

### HTMX Endpoints

| Method | Path                                        | Description                             | Response                                    |
|--------|---------------------------------------------|-----------------------------------------|---------------------------------------------|
| GET    | `/api/tasks`                                | Fetch tasks with filters                | HTML fragment - Task table rows             |
| POST   | `/api/tasks`                                | Create new task                         | HTML fragment - Success notification        |
| GET    | `/api/tasks/{id}`                           | Fetch task details                      | HTML fragment - Task details                |
| PUT    | `/api/tasks/{id}`                           | Update task                             | HTML fragment - Updated details             |
| DELETE | `/api/tasks/{id}`                           | Delete task                             | HTML fragment - Success notification        |
| GET    | `/api/tasks/{id}/materials`                 | Fetch materials for task                | HTML fragment - Materials table rows        |
| POST   | `/api/tasks/{id}/materials`                 | Add material to task                    | HTML fragment - Success notification        |
| PUT    | `/api/tasks/{id}/materials/{material_id}`   | Update material usage                   | HTML fragment - Updated material row        |
| DELETE | `/api/tasks/{id}/materials/{material_id}`   | Remove material from task               | HTML fragment - Success notification        |
| PUT    | `/api/tasks/{id}/complete`                  | Mark task as completed                  | HTML fragment - Updated task status         |
| GET    | `/api/tasks/by-site/{site_id}`              | Fetch tasks by site                     | HTML fragment - Task table rows             |
| GET    | `/api/tasks/by-brigade/{brigade_id}`        | Fetch tasks by brigade                  | HTML fragment - Task table rows             |
| GET    | `/api/tasks/overdue`                        | Fetch overdue tasks                     | HTML fragment - Task table rows             |
| GET    | `/api/materials`                            | Fetch materials with filters            | HTML fragment - Material table rows         |
| POST   | `/api/materials`                            | Create new material                     | HTML fragment - Success notification        |
| GET    | `/api/materials/{id}`                       | Fetch material details                  | HTML fragment - Material details            |
| PUT    | `/api/materials/{id}`                       | Update material                         | HTML fragment - Updated details             |
| DELETE | `/api/materials/{id}`                       | Delete material                         | HTML fragment - Success notification        |
| GET    | `/api/materials/with-excesses`              | Fetch materials with estimate excesses  | HTML fragment - Material table rows         |

## 9. Reports & Analytics Endpoints

### Page Endpoints

| Method | Path                               | Description                     | Response                          |
|--------|------------------------------------|---------------------------------|-----------------------------------|
| GET    | `/reports/departments`             | Department reports page         | HTML - Department reports page    |
| GET    | `/reports/sites`                   | Site reports page               | HTML - Site reports page          |
| GET    | `/reports/brigades`                | Brigade reports page            | HTML - Brigade reports page       |

### HTMX Endpoints

| Method | Path                                                | Description                                  | Response                                      |
|--------|-----------------------------------------------------|----------------------------------------------|-----------------------------------------------|
| GET    | `/api/reports/departments/{id}/sites`               | Generate department sites report             | HTML fragment - Department sites report       |
| GET    | `/api/reports/departments/{id}/equipment`           | Generate department equipment report         | HTML fragment - Department equipment report   |
| GET    | `/api/reports/departments/{id}/tasks`               | Generate department tasks report             | HTML fragment - Department tasks report       |
| GET    | `/api/reports/departments/{id}/materials`           | Generate department materials report         | HTML fragment - Department materials report   |
| GET    | `/api/reports/sites/{id}/schedule`                  | Generate site schedule report                | HTML fragment - Site schedule report          |
| GET    | `/api/reports/sites/{id}/materials`                 | Generate site materials report               | HTML fragment - Site materials report         |
| GET    | `/api/reports/sites/{id}/equipment`                 | Generate site equipment report               | HTML fragment - Site equipment report         |
| GET    | `/api/reports/sites/{id}/brigades`                  | Generate site brigades report                | HTML fragment - Site brigades report          |
| GET    | `/api/reports/sites/{id}/construction`              | Generate site construction report            | HTML fragment - Site construction report      |
| GET    | `/api/reports/brigades/{id}/tasks`                  | Generate brigade tasks report                | HTML fragment - Brigade tasks report          |
| GET    | `/api/reports/brigades/{id}/sites`                  | Generate brigade sites report                | HTML fragment - Brigade sites report          |
| GET    | `/api/reports/tasks/overdue`                        | Generate overdue tasks report                | HTML fragment - Overdue tasks report          |
| GET    | `/api/reports/materials/excesses`                   | Generate material excesses report            | HTML fragment - Material excesses report      |

## Form Helper Endpoints

These endpoints provide dynamic form functionality with HTMX:

| Method | Path                                     | Description                                   | Response                                    |
|--------|------------------------------------------|-----------------------------------------------|---------------------------------------------|
| GET    | `/api/selectors/departments`             | Get department selector options               | HTML fragment - Department options          |
| GET    | `/api/selectors/areas`                   | Get area selector options                     | HTML fragment - Area options                |
| GET    | `/api/selectors/areas-by-department/{id}`| Get areas filtered by department              | HTML fragment - Area options                |
| GET    | `/api/selectors/clients`                 | Get client selector options                   | HTML fragment - Client options              |
| GET    | `/api/selectors/workers`                 | Get worker selector options                   | HTML fragment - Worker options              |
| GET    | `/api/selectors/brigades`                | Get brigade selector options                  | HTML fragment - Brigade options             |
| GET    | `/api/selectors/sites`                   | Get site selector options                     | HTML fragment - Site options                |
| GET    | `/api/selectors/materials`               | Get material selector options                 | HTML fragment - Material options            |
| GET    | `/api/validation/form/{form_type}`       | Validate form fields                          | HTML fragment - Validation messages         |

## Error Handling

All API endpoints should use appropriate HTTP status codes:

- 200: Success
- 201: Created
- 400: Bad Request (validation errors)
- 404: Not Found
- 500: Server Error

When errors occur in HTMX endpoints, return HTML fragments with error messages that can be displayed within the UI.