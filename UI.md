# Onyx Horizons UI/UX Design

## Overview

The Onyx Horizons informational system UI is designed to be simple, functional, and efficient. The interface uses HTMX for interactivity and Tailwind CSS for styling, eliminating the need for custom JavaScript.

## Layout Structure

### Global Layout
- **Header**: Contains the system logo, navigation menu, and user information/authentication controls
- **Sidebar**: Context-sensitive navigation that changes based on the current section
- **Main Content Area**: Displays the primary content, forms, tables, and query results
- **Footer**: Contains copyright information, links to documentation, and contact information

## Navigation

- **Top-level Navigation**: Main sections of the application (Dashboard, Entities, Reports, etc.)
- **Breadcrumb Trail**: Shows the current navigation path for easy backtracking
- **Entity Navigation**: Quick access to related entities from any detail page

## Entity Management Interface

Each entity in the system follows a consistent interface pattern:

### List View
- Sortable columns with headers that trigger HTMX sorting requests
- Pagination controls (Previous/Next/Page Numbers)
- Quick filters for common queries
- Advanced filter collapsible section
- Bulk action support (Delete, Export, etc.)
- "Create New" button prominently displayed

```html
<!-- List view example with HTMX -->
<div class="container mx-auto py-6">
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-2xl font-bold">[Entity Name] List</h1>
    <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
            hx-get="/[entity]/create"
            hx-target="#main-content">
      Create New
    </button>
  </div>
  
  <!-- Quick Filters -->
  <div class="mb-4 flex gap-2">
    <button class="border px-3 py-1 rounded hover:bg-gray-100"
            hx-get="/[entity]?filter=active"
            hx-target="#entity-list">
      Active Only
    </button>
    <!-- Additional filter buttons -->
  </div>
  
  <!-- Advanced Filters -->
  <div class="mb-4 border rounded p-4">
    <div class="flex justify-between items-center mb-2">
      <h3 class="font-medium">Advanced Filters</h3>
      <button class="text-sm text-blue-600"
              hx-get="/[entity]/filters"
              hx-target="#filter-form">
        Toggle
      </button>
    </div>
    <form id="filter-form" class="space-y-2"
          hx-get="/[entity]"
          hx-trigger="change delay:500ms"
          hx-target="#entity-list">
      <!-- Filter form fields -->
    </form>
  </div>
  
  <!-- Results Table -->
  <div id="entity-list" class="bg-white shadow overflow-hidden rounded">
    <table class="min-w-full divide-y divide-gray-200">
      <thead class="bg-gray-50">
        <tr>
          <!-- Column headers with HTMX sorting -->
          <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer"
              hx-get="/[entity]?sort=name"
              hx-target="#entity-list">
            Name
          </th>
          <!-- Additional columns -->
        </tr>
      </thead>
      <tbody class="bg-white divide-y divide-gray-200">
        <!-- Table rows with entities -->
      </tbody>
    </table>
    
    <!-- Pagination -->
    <div class="px-6 py-3 flex items-center justify-between border-t">
      <div class="flex-1 flex justify-between items-center">
        <button class="border px-4 py-2 rounded"
                hx-get="/[entity]?page=prev"
                hx-target="#entity-list">
          Previous
        </button>
        <span class="text-sm text-gray-700">Page <span id="current-page">1</span> of <span id="total-pages">10</span></span>
        <button class="border px-4 py-2 rounded"
                hx-get="/[entity]?page=next"
                hx-target="#entity-list">
          Next
        </button>
      </div>
    </div>
  </div>
</div>
```

### Detail View
- Full entity information display
- Edit button that transforms the view into an editable form
- Delete button with confirmation prompt
- Links to related entities
- Action buttons specific to the entity type
- History/audit log of changes (if applicable)

```html
<!-- Detail view example with HTMX -->
<div class="container mx-auto py-6">
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-2xl font-bold">[Entity Name] Details</h1>
      <div class="text-sm text-gray-500">ID: 12345</div>
    </div>
    <div class="space-x-2">
      <button class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
              hx-get="/[entity]/12345/edit"
              hx-target="#main-content">
        Edit
      </button>
      <button class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded"
              hx-delete="/[entity]/12345"
              hx-confirm="Are you sure you want to delete this item?"
              hx-target="#main-content"
              hx-push-url="/[entity]">
        Delete
      </button>
    </div>
  </div>
  
  <!-- Entity Fields -->
  <div class="bg-white shadow overflow-hidden rounded-lg">
    <div class="px-4 py-5 sm:p-6">
      <dl class="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
        <div class="sm:col-span-1">
          <dt class="text-sm font-medium text-gray-500">Name</dt>
          <dd class="mt-1 text-sm text-gray-900">Entity Name Value</dd>
        </div>
        <!-- Additional fields -->
      </dl>
    </div>
  </div>
  
  <!-- Related Entities -->
  <div class="mt-6">
    <h2 class="text-xl font-semibold mb-3">Related Items</h2>
    <div class="bg-white shadow overflow-hidden rounded-lg">
      <ul class="divide-y divide-gray-200">
        <li class="px-6 py-4 hover:bg-gray-50">
          <a href="/related-entity/456" class="flex justify-between items-center">
            <div>Related Entity Name</div>
            <div class="text-sm text-gray-500">View Details →</div>
          </a>
        </li>
        <!-- Additional related entities -->
      </ul>
    </div>
  </div>
</div>
```

### Create/Edit Forms
- Validation inline with HTMX validation extension
- Auto-save for drafts
- Cancel button that returns to previous view
- Field grouping for better organization
- Dynamic form elements that appear/disappear based on other selections

```html
<!-- Create/Edit form example with HTMX -->
<div class="container mx-auto py-6">
  <h1 class="text-2xl font-bold mb-6">[Create/Edit] [Entity Name]</h1>
  
  <form hx-post="/[entity]"
        hx-target="#main-content"
        hx-push-url="true"
        class="bg-white shadow rounded-lg p-6">
    
    <!-- Form Fields -->
    <div class="space-y-6">
      <div>
        <label for="name" class="block text-sm font-medium text-gray-700">Name</label>
        <input type="text" name="name" id="name" 
               class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"
               hx-post="/validate/name"
               hx-trigger="change"
               hx-target="#name-error">
        <div id="name-error" class="text-red-600 text-sm mt-1"></div>
      </div>
      
      <!-- Dynamic content example -->
      <div>
        <label for="type" class="block text-sm font-medium text-gray-700">Type</label>
        <select id="type" name="type"
                class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"
                hx-get="/[entity]/type-fields"
                hx-target="#dynamic-fields"
                hx-trigger="change">
          <option value="">Select a type</option>
          <option value="type1">Type 1</option>
          <option value="type2">Type 2</option>
        </select>
      </div>
      
      <!-- Dynamic fields will be loaded here -->
      <div id="dynamic-fields"></div>
      
      <!-- Related entity selection -->
      <div>
        <label for="related" class="block text-sm font-medium text-gray-700">Related Entity</label>
        <div class="flex space-x-2">
          <select id="related" name="related" 
                  class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
            <!-- Options loaded from backend -->
          </select>
          <button type="button"
                  hx-get="/related-entity/search"
                  hx-target="#related-search-modal"
                  class="bg-gray-200 px-3 rounded-md">
            Search
          </button>
        </div>
        <div id="related-search-modal"></div>
      </div>
    </div>
    
    <!-- Form Actions -->
    <div class="mt-8 flex justify-end space-x-3">
      <button type="button" 
              class="bg-white border border-gray-300 px-4 py-2 rounded text-gray-700"
              hx-get="/[entity]">
        Cancel
      </button>
      <button type="submit" 
              class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded">
        Save
      </button>
    </div>
  </form>
</div>
```

### Delete Confirmation
- Clear warning about consequences
- Option to cancel
- Explanation of what related data might be affected

## Dashboard Interface

- Summary cards showing key metrics
- Recent activity/changes
- Quick access to common tasks
- Saved queries/bookmarks
- Notifications/alerts

```html
<!-- Dashboard example with HTMX -->
<div class="container mx-auto py-6">
  <h1 class="text-2xl font-bold mb-6">Dashboard</h1>
  
  <!-- Metrics Cards -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
    <div class="bg-white rounded-lg shadow p-6">
      <div class="text-sm text-gray-500">Total [Entities]</div>
      <div class="text-3xl font-bold">1,234</div>
      <div class="text-sm text-green-600 mt-2">+12% from last month</div>
    </div>
    <!-- Additional metrics cards -->
  </div>
  
  <!-- Quick Access -->
  <div class="bg-white rounded-lg shadow p-6 mb-6">
    <h2 class="text-lg font-semibold mb-4">Quick Access</h2>
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <a href="/entity/create" class="p-4 border rounded-lg text-center hover:bg-gray-50">
        <div class="text-xl mb-2">+</div>
        <div>New Entity</div>
      </a>
      <!-- Additional quick access buttons -->
    </div>
  </div>
  
  <!-- Recent Activity -->
  <div class="bg-white rounded-lg shadow p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-semibold">Recent Activity</h2>
      <button class="text-sm text-blue-600"
              hx-get="/activity?refresh=true"
              hx-target="#activity-list">
        Refresh
      </button>
    </div>
    <div id="activity-list">
      <ul class="divide-y divide-gray-200">
        <!-- Activity items -->
      </ul>
    </div>
  </div>
</div>
```

## Query Interfaces

### Basic Search
- Simple search field in the header of every page
- Quick filters for common queries
- Partial text matching

### Advanced Search
- Multiple fields for specific entity properties
- Date range selectors
- Dropdown for categorical filters
- Toggle between AND/OR conditions
- Save search criteria option

```html
<!-- Advanced search example with HTMX -->
<div class="container mx-auto py-6">
  <h1 class="text-2xl font-bold mb-6">Advanced Search</h1>
  
  <div class="bg-white rounded-lg shadow p-6">
    <form hx-get="/search/results"
          hx-target="#search-results"
          class="space-y-4">
      
      <!-- Entity Type Selection -->
      <div>
        <label class="block text-sm font-medium text-gray-700">Entity Type</label>
        <select name="entity_type" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2"
                hx-get="/search/fields"
                hx-target="#dynamic-search-fields"
                hx-trigger="change">
          <option value="">All Types</option>
          <option value="type1">Type 1</option>
          <option value="type2">Type 2</option>
        </select>
      </div>
      
      <!-- Dynamic search fields based on entity type -->
      <div id="dynamic-search-fields"></div>
      
      <!-- Date Range -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700">From Date</label>
          <input type="date" name="date_from" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700">To Date</label>
          <input type="date" name="date_to" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2">
        </div>
      </div>
      
      <!-- Search Logic -->
      <div>
        <label class="block text-sm font-medium text-gray-700">Search Logic</label>
        <div class="mt-1 flex items-center space-x-4">
          <label class="inline-flex items-center">
            <input type="radio" name="logic" value="and" checked class="form-radio">
            <span class="ml-2">Match All (AND)</span>
          </label>
          <label class="inline-flex items-center">
            <input type="radio" name="logic" value="or" class="form-radio">
            <span class="ml-2">Match Any (OR)</span>
          </label>
        </div>
      </div>
      
      <!-- Actions -->
      <div class="flex justify-between">
        <button type="button"
                class="text-blue-600 underline"
                hx-get="/search/save-form"
                hx-target="#save-search-modal">
          Save This Search
        </button>
        <div class="space-x-2">
          <button type="reset" class="bg-gray-200 px-4 py-2 rounded">
            Reset
          </button>
          <button type="submit" class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded">
            Search
          </button>
        </div>
      </div>
    </form>
    
    <div id="save-search-modal"></div>
  </div>
  
  <!-- Search Results -->
  <div id="search-results" class="mt-6">
    <!-- Results will be loaded here -->
  </div>
</div>
```

### Specialized Query Interfaces
Based on the queries mentioned in TASK.md, specialized interfaces will be created with appropriate filters and parameters.

## Responsive Design

The UI will be responsive across different devices with these breakpoints:
- Mobile: < 640px
- Tablet: 640px - 1024px
- Desktop: > 1024px

Mobile views will prioritize:
- Collapsible navigation
- Stackable content blocks
- Simplified tables with fewer columns
- Touch-friendly controls

## Interactive Elements with HTMX

Key interactive features implemented with HTMX:
- Infinite scrolling lists
- In-place editing
- Form validation
- Modal dialogs
- Dynamic filtering
- Sorting
- Pagination
- Progress indicators
- Lazy loading of content
- Live updates

## Notifications & Feedback

- Toast messages for actions (success, error, warnings)
- Form validation messages
- In-progress indicators for long-running operations
- Empty state displays with helpful guidance

## Accessibility Features

- Proper ARIA labels
- Keyboard navigation
- Focus management
- High contrast mode support
- Screen reader compatibility

## Implementation Notes

### HTMX Usage
- Use `hx-get`, `hx-post`, `hx-put`, and `hx-delete` for CRUD operations
- Use `hx-target` to update specific parts of the page
- Use `hx-trigger` for custom events (e.g., delayed searches)
- Use `hx-swap` to control how content is inserted
- Use `hx-push-url` for updating browser history

### Tailwind CSS Classes
- Use utility classes for consistent spacing: `p-4`, `m-2`, `gap-3`, etc.
- Use flex and grid for layouts: `flex`, `grid grid-cols-3`, etc.
- Use responsive prefixes for different screen sizes: `md:`, `lg:`, etc.
- Use color utilities with consistent palette: `text-blue-600`, `bg-gray-100`, etc.
- Use transition utilities for hover states: `hover:bg-gray-50`, etc.
